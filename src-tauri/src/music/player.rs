use anyhow::Result;
use rodio::{
  cpal::{default_host, traits::HostTrait},
  Device, DeviceTrait,
};
use serde::{Deserialize, Serialize};
use std::{
  fs::{metadata, OpenOptions},
  io::Write,
  path::PathBuf,
  sync::{
    atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    Arc, LazyLock, RwLock,
  },
  time::Duration,
};
use tauri::{async_runtime, http::Method, ipc::Channel, Emitter};
use tauri_plugin_http::reqwest::Response;
use tokio::time;
use tokio_stream::StreamExt;

use crate::{
  app::{config::APP_HANDLE, dir::AppDir},
  http::client::HttpClient,
  log_err,
  music::{audio::Audio, stream::StreamFile},
  utils::{logger::LogErrExt, tools::is_valid_hash},
};

// 获取文件句柄超时
const FILE_TIMEOUT: Duration = Duration::from_millis(5000);
// 监测设备的间隔
const DEVICE_INTERVAL: Duration = Duration::from_millis(1000);
// 播放进度的获取间隔
const PLAY_INTERVAL: Duration = Duration::from_millis(16);
// 下载进度的获取间隔
const DOWNLOAD_INTERVAL: Duration = Duration::from_millis(100);
// 最小读取文件大小
const MIN_READ_SIZE: u64 = 128 * 1024;
// 淡入淡出的间隔
const FADE_INTERVAL: Duration = Duration::from_millis(16);
// 淡入淡出的总时长
const FADE_DURATION: Duration = Duration::from_millis(300);

static PLAYER: LazyLock<Player> = LazyLock::new(|| Player::default());

struct Player {
  audio: Arc<RwLock<Audio>>,           // 音频实例
  audio_size: Arc<AtomicU64>,          // 音频文件大小
  play_channel_id: Arc<AtomicU64>,     // 播放进度的id
  download_channel_id: Arc<AtomicU64>, // 下载进度的id
  loading_id: Arc<AtomicU64>,          // 加载url的id
  is_downloading: Arc<AtomicBool>,     // 是否正在下载
  downloaded_size: Arc<AtomicU64>,     // 已下载的文件大小
  target_volume: Arc<AtomicU32>,       // 目标音量(0~1的f32位表示)
  fade_id: Arc<AtomicU64>,             // 淡入淡出的任务id
}

impl Default for Player {
  fn default() -> Self {
    let audio = Audio::new().expect("构建播放器失败");

    Self {
      audio: Arc::new(RwLock::new(audio)),
      audio_size: Arc::new(AtomicU64::new(0)),
      play_channel_id: Arc::new(AtomicU64::new(0)),
      download_channel_id: Arc::new(AtomicU64::new(0)),
      loading_id: Arc::new(AtomicU64::new(0)),
      is_downloading: Arc::new(AtomicBool::new(false)),
      downloaded_size: Arc::new(AtomicU64::new(0)),
      target_volume: Arc::new(AtomicU32::new(1.0f32.to_bits())),
      fade_id: Arc::new(AtomicU64::new(0)),
    }
  }
}

impl Player {
  /// ## 监测输出设备
  ///
  /// 默认输出设备变化时自动重载播放器设备
  fn monitor_output_device(&self) {
    let app_handle = APP_HANDLE.get().expect("无法获取 APP_HANDLE");
    let audio = self.audio.clone();

    async_runtime::spawn(async move {
      let default_host = default_host();
      let mut old_did = default_host
        .default_output_device()
        .and_then(|d| d.id().ok());

      let mut interval = time::interval(DEVICE_INTERVAL);

      loop {
        interval.tick().await;

        let new_device = default_host.default_output_device();
        let new_did = new_device.as_ref().and_then(|d| d.id().ok());

        match (&old_did, &new_did) {
          (Some(od), Some(nd)) if od == nd => continue,
          _ => {
            if let Some(device) = new_device {
              if let Ok(mut audio_writer) = audio.write() {
                match audio_writer.reload_device(device) {
                  Ok(_) => {
                    let _ = app_handle.emit("music:reload_device", true);
                  }
                  Err(e) => log_err!("monitor_output_device", e, "重载播放设备失败"),
                }
              }
            }

            old_did = new_did;
          }
        }
      }
    });
  }

  /// ## 下载文件
  ///
  /// ### 必选参数
  /// * `current_loading_id` - 当前加载的id, 用于中止无效的下载
  /// * `file_path` - 文件保存路径
  /// * `response` - 音频响应
  fn download_file(
    &self,
    current_loading_id: u64,
    file_path: &PathBuf,
    response: Response,
  ) -> anyhow::Result<()> {
    let loading_id = self.loading_id.clone();
    let is_downloading = self.is_downloading.clone();
    let downloaded_size = self.downloaded_size.clone();

    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(file_path)?;

    async_runtime::spawn(async move {
      is_downloading.store(true, Ordering::Release);

      let mut stream = response.bytes_stream();
      while let Some(chunk_result) = stream.next().await {
        if loading_id.load(Ordering::Acquire) != current_loading_id {
          break;
        }

        match chunk_result {
          Ok(chunk) => {
            if let Err(e) = file.write_all(&chunk) {
              log_err!("download_file", e, "写入失败");
              break;
            }

            if let Err(e) = file.flush() {
              log_err!("download_file", e, "flush失败");
              break;
            }

            downloaded_size.fetch_add(chunk.len() as u64, Ordering::AcqRel);
          }
          Err(e) => {
            log_err!("download_file", e, "读取音频流失败");
            break;
          }
        }
      }

      is_downloading.store(false, Ordering::Release);
    });

    Ok(())
  }

  /// ## 加载流式文件
  ///
  /// ### 必选参数
  /// * `current_loading_id` - 当前加载的id, 用于中止无效的加载
  /// * `file_path` - 文件路径
  /// * `file_size` - 文件总大小
  fn load_stream(&self, current_loading_id: u64, file_path: &PathBuf, file_size: u64) {
    let file_path = file_path.clone();
    let audio = self.audio.clone();
    let loading_id = self.loading_id.clone();
    let downloaded_size = self.downloaded_size.clone();

    async_runtime::spawn(async move {
      let wait_result = time::timeout(FILE_TIMEOUT, async {
        loop {
          if loading_id.load(Ordering::Acquire) != current_loading_id {
            break None;
          }

          // 下载足够的数据时再读取
          if downloaded_size.load(Ordering::Acquire) < MIN_READ_SIZE {
            time::sleep(DOWNLOAD_INTERVAL).await;
            continue;
          }

          match OpenOptions::new().read(true).open(&file_path) {
            Ok(file) => break Some(file),
            Err(_) => time::sleep(DOWNLOAD_INTERVAL).await,
          }
        }
      })
      .await;

      match wait_result {
        Ok(Some(file)) => {
          if loading_id.load(Ordering::Acquire) != current_loading_id {
            return;
          }

          let stream_file = StreamFile::new(file, file_size, downloaded_size);

          let mut audio_writer = audio.write().unwrap_or_else(|e| e.into_inner());

          if let Err(e) = audio_writer.load_from_stream(stream_file, file_size) {
            log_err!("load_stream", e, "加载音频流失败")
          }
        }
        _ => {}
      }
    });
  }

  /// ## 淡入
  ///
  /// 先播放, 再将音量从当前值渐变到目标音量
  fn fade_in(&self) {
    let target = f32::from_bits(self.target_volume.load(Ordering::Acquire));

    self.audio.read().unwrap_or_else(|e| e.into_inner()).play();

    self.fade_to(target, false);
  }

  /// ## 淡出
  ///
  /// 将音量从当前值渐变到 0, 结束后暂停
  fn fade_out(&self) {
    self.fade_to(0.0, true);
  }

  /// ## 平滑调整音量
  ///
  /// 在固定时长内将音量渐变到目标值, 结束后可按需暂停
  ///
  /// ### 必选参数
  /// * `target` - 目标音量, 取值范围为 0 ~ 1
  /// * `pause_after` - 渐变结束后是否暂停
  fn fade_to(&self, target: f32, pause_after: bool) {
    let audio = self.audio.clone();
    let fade_id = self.fade_id.clone();
    // 保存 fade_id 快照, 使旧的渐变任务失效
    let current_id = fade_id.fetch_add(1, Ordering::AcqRel) + 1;

    async_runtime::spawn(async move {
      let start = {
        let audio_reader = audio.read().unwrap_or_else(|e| e.into_inner());
        audio_reader.volume()
      };

      let steps = fade_steps();

      for step in 1..=steps {
        if current_id != fade_id.load(Ordering::Acquire) {
          return;
        }

        let volume = fade_volume(start, target, step, steps);
        {
          let audio_reader = audio.read().unwrap_or_else(|e| e.into_inner());
          audio_reader.set_volume(volume);
        }

        time::sleep(FADE_INTERVAL).await;
      }

      if pause_after {
        let audio_reader = audio.read().unwrap_or_else(|e| e.into_inner());
        audio_reader.pause();
      }
    });
  }
}

pub struct MusicPlayer;

impl MusicPlayer {
  /// ## 初始化播放器
  ///
  /// 开始监测默认输出设备
  pub fn init() {
    PLAYER.monitor_output_device();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
  id: String,
  name: String,
}

impl TryFrom<&Device> for DeviceInfo {
  type Error = anyhow::Error;

  fn try_from(device: &Device) -> Result<Self, Self::Error> {
    let id = device.id()?.to_string();
    let description = device.description()?;

    let name = match description.driver() {
      Some(driver) => format!("{}({})", description.name(), driver),
      None => description.name().to_string(),
    };

    Ok(DeviceInfo { id, name })
  }
}

#[tauri::command]
/// ## 获取当前输出设备
pub fn music_player_get_device() -> Result<DeviceInfo, String> {
  let audio_reader = PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  DeviceInfo::try_from(audio_reader.output_device())
    .log_command("music_player_get_device", "获取当前输出设备失败")
}

#[tauri::command]
/// ## 设置当前输出设备
///
/// ### 必选参数
/// * `id` - 设备id
pub fn music_player_set_device(id: &str) -> Result<(), String> {
  let mut audio_writer = PLAYER.audio.write().unwrap_or_else(|e| e.into_inner());

  let devices = audio_writer
    .output_devices()
    .log_command("music_player_set_device", "获取输出设备列表失败")?;
  let device = devices
    .into_iter()
    .find(|d| d.id().is_ok_and(|i| i.to_string() == id))
    .ok_or("未获取到输出设备")?;

  audio_writer
    .reload_device(device)
    .log_command("music_player_set_device", "重载播放设备失败")
}

#[tauri::command]
/// ## 获取所有输出设备
pub fn music_player_get_devices() -> Result<Vec<DeviceInfo>, String> {
  let audio_reader = PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  let devices = audio_reader
    .output_devices()
    .log_command("music_player_get_devices", "获取输出设备列表失败")?
    .into_iter()
    .filter_map(|d| DeviceInfo::try_from(&d).ok())
    .collect();

  Ok(devices)
}

#[tauri::command]
/// ## 从本地文件加载音频
///
/// ### 必选参数
/// * `path` - 本地音频文件路径
pub fn music_player_load_file(path: String) -> Result<(), String> {
  let mut audio_writer = PLAYER.audio.write().unwrap_or_else(|e| e.into_inner());

  audio_writer
    .load_from_file(&path)
    .log_command("music_player_load_file", "加载文件音频失败")
}

#[tauri::command]
/// ## 从网络 url 加载音频
///
/// ### 必选参数
/// * `url` - 音频的网络地址
/// * `hash` - 音频 hash, 用于本地缓存文件的命名
pub async fn music_player_load_url(url: String, hash: String) -> Result<(), String> {
  // 验证 hash 参数的合法性
  if !is_valid_hash(&hash) {
    return Err("不合法的hash".to_string());
  }

  let response = HttpClient::new(url, Method::GET)
    .send()
    .await
    .log_command("music_player_load_url", "请求音频资源失败")?;
  let file_size = response.content_length().unwrap_or_default();
  if file_size == 0 {
    return Err("无法获取音频长度".to_string());
  }

  // 保存id快照
  let current_loading_id = PLAYER.loading_id.fetch_add(1, Ordering::AcqRel) + 1;
  // 重置所有状态
  PLAYER.is_downloading.store(true, Ordering::Release);
  PLAYER.downloaded_size.store(0, Ordering::Release);
  PLAYER.audio_size.store(file_size, Ordering::Release);

  let file_path = AppDir::audio().join(&hash);

  // 检查文件是否存在且全量数据
  if let Ok(metadata) = metadata(&file_path) {
    if metadata.len() == file_size {
      PLAYER.is_downloading.store(false, Ordering::Release);
      PLAYER.downloaded_size.store(file_size, Ordering::Release);

      return music_player_load_file(file_path.to_string_lossy().into_owned());
    }
  }

  let _ = PLAYER.download_file(current_loading_id, &file_path, response);
  PLAYER.load_stream(current_loading_id, &file_path, file_size);

  Ok(())
}

#[tauri::command]
/// ## 监测下载进度
///
/// ### 必选参数
/// * `channel` - 下载进度的通道, 返回值为 0 ~ 1 的进度值
pub fn music_player_monitor_download(channel: Channel<f32>) {
  let download_channel_id = PLAYER.download_channel_id.clone();
  let downloaded_size = PLAYER.downloaded_size.clone();
  let audio_size = PLAYER.audio_size.clone();

  // 保存 download_channel_id 快照
  let current_channel_id = download_channel_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = time::interval(DOWNLOAD_INTERVAL);

    loop {
      if current_channel_id != download_channel_id.load(Ordering::Acquire) {
        break;
      }

      interval.tick().await;

      let downloaded_size = downloaded_size.load(Ordering::Acquire) as f32;
      let audio_size = audio_size.load(Ordering::Acquire) as f32;

      if downloaded_size == 0.0 || audio_size == 0.0 {
        continue;
      }

      let _ = channel.send(downloaded_size / audio_size);
    }
  });
}

#[tauri::command]
/// ## 监测播放进度
///
/// ### 必选参数
/// * `channel` - 播放进度的通道, 返回值为已播放的秒数
pub fn music_player_monitor_play(channel: Channel<f32>) {
  let audio = PLAYER.audio.clone();
  let play_channel_id = PLAYER.play_channel_id.clone();
  let current_channel_id = play_channel_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = time::interval(PLAY_INTERVAL);

    // TODO: 即使使用的原子类型判断, 前端页面刷新时旧的循环仍然会执行几次
    // 定义了stop函数,在前端页面卸载时调用也不行, 待优化
    loop {
      if current_channel_id != play_channel_id.load(Ordering::Acquire) {
        break;
      }

      interval.tick().await;

      let progress = {
        let audio_reader = audio.read().unwrap_or_else(|e| e.into_inner());

        if !audio_reader.paused() {
          audio_reader.get_pos().as_secs_f32()
        } else {
          continue;
        }
      };

      let _ = channel.send(progress);
    }
  });
}

#[tauri::command]
/// ## 播放
pub fn music_player_play() {
  PLAYER.fade_in()
}

#[tauri::command]
/// ## 暂停
pub fn music_player_pause() {
  PLAYER.fade_out()
}

#[tauri::command]
/// ## 停止
pub async fn music_player_stop() {
  let mut audio_writer = PLAYER.audio.write().unwrap_or_else(|e| e.into_inner());

  audio_writer.stop()
}

#[tauri::command]
/// ## 跳转播放进度
///
/// ### 必选参数
/// * `pos` - 播放位置, 单位为秒
pub fn music_player_seek(pos: f32) -> Result<(), String> {
  let audio_reader = PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio_reader
    .try_seek(Duration::from_secs_f32(pos))
    .log_command("music_player_seek", "跳转失败")
}

#[tauri::command]
/// ## 设置音量
///
/// ### 必选参数
/// * `volume` - 音量, 取值范围为 0 ~ 100
pub fn music_player_set_volume(volume: f32) {
  let target = volume / 100.0;
  PLAYER.target_volume.store(target.to_bits(), Ordering::Release);

  let audio_reader = PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio_reader.set_volume(target)
}

/// 计算淡入淡出的步数
fn fade_steps() -> u32 {
  (FADE_DURATION.as_millis() / FADE_INTERVAL.as_millis()).max(1) as u32
}

/// 计算淡入淡出第 `step` 步的音量, 在 `start` 与 `target` 之间线性插值
fn fade_volume(start: f32, target: f32, step: u32, steps: u32) -> f32 {
  start + (target - start) * (step as f32 / steps as f32)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fade_steps_is_duration_div_interval() {
    // 300ms / 16ms = 18.75, 截断为 18 步
    assert_eq!(fade_steps(), 18);
  }

  #[test]
  fn fade_volume_first_step_is_between_start_and_target() {
    let steps = fade_steps();
    let volume = fade_volume(0.0, 1.0, 1, steps);

    assert!(volume > 0.0);
    assert!(volume < 1.0);
  }

  #[test]
  fn fade_volume_final_step_reaches_target() {
    let steps = fade_steps();
    assert_eq!(fade_volume(0.0, 1.0, steps, steps), 1.0);
  }

  #[test]
  fn fade_volume_fade_out_reaches_zero() {
    let steps = fade_steps();
    assert_eq!(fade_volume(0.8, 0.0, steps, steps), 0.0);
  }

  #[test]
  fn fade_volume_is_monotonic_fade_in() {
    let steps = fade_steps();
    let mut prev = f32::NEG_INFINITY;

    for step in 1..=steps {
      let volume = fade_volume(0.0, 1.0, step, steps);
      assert!(volume >= prev, "音量应单调递增");
      prev = volume;
    }
  }

  #[test]
  fn fade_volume_is_monotonic_fade_out() {
    let steps = fade_steps();
    let mut prev = f32::INFINITY;

    for step in 1..=steps {
      let volume = fade_volume(0.8, 0.0, step, steps);
      assert!(volume <= prev, "音量应单调递减");
      prev = volume;
    }
  }
}
