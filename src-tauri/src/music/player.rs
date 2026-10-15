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
    atomic::{AtomicU32, AtomicU64, Ordering},
    LazyLock, RwLock,
  },
  time::Duration,
};
use tauri::{async_runtime, ipc::Channel, Emitter};
use tauri_plugin_http::reqwest::{Method, Response};
use tokio::time::{interval, sleep, timeout};
use tokio_stream::StreamExt;

use crate::{
  app::{config::Config, dir::Dir},
  http::client::Client,
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
// 淡入淡出的总时长
const FADE_DURATION: Duration = Duration::from_millis(300);
// 淡入淡出的间隔
const FADE_INTERVAL: Duration = Duration::from_millis(16);
// 最小读取文件大小
const MIN_READ_SIZE: u64 = 128 * 1024;

static MUSIC_PLAYER: LazyLock<MusicPlayer> = LazyLock::new(MusicPlayer::default);

struct MusicPlayer {
  audio: RwLock<Audio>,    // 音频实例
  audio_volume: AtomicU32, // 音频音量(0~1的f32位表示)

  audio_size: AtomicU64,      // 音频文件大小
  downloaded_size: AtomicU64, // 下载文件大小

  play_id: AtomicU64,     // 播放进度的id
  download_id: AtomicU64, // 下载进度的id
  loading_id: AtomicU64,  // 加载url的id
  fade_id: AtomicU64,     // 淡入淡出的id
}

impl Default for MusicPlayer {
  fn default() -> Self {
    let audio = Audio::new().expect("构建播放器失败");

    Self {
      audio: RwLock::new(audio),
      audio_volume: AtomicU32::new(1.0f32.to_bits()),
      audio_size: AtomicU64::new(0),
      downloaded_size: AtomicU64::new(0),

      play_id: AtomicU64::new(0),
      download_id: AtomicU64::new(0),
      loading_id: AtomicU64::new(0),
      fade_id: AtomicU64::new(0),
    }
  }
}

pub struct Player;

impl Player {
  /// ## 初始化播放器
  ///
  /// 开始监测默认输出设备
  pub fn init() {
    Self::monitor_output_device();
  }

  /// ## 监测输出设备
  ///
  /// 默认输出设备变化时自动重载播放器设备
  fn monitor_output_device() {
    async_runtime::spawn(async move {
      let default_host = default_host();
      let mut old_did = {
        let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

        audio.output_device().id().ok()
      };

      let mut interval = interval(DEVICE_INTERVAL);

      loop {
        interval.tick().await;

        let new_device = default_host.default_output_device();
        let new_did = new_device.as_ref().and_then(|d| d.id().ok());

        match (&old_did, &new_did) {
          (Some(od), Some(nd)) if od == nd => continue,
          _ => {
            if let Some(new_device) = new_device {
              if let Ok(mut audio) = MUSIC_PLAYER.audio.write() {
                match audio.reload_device(new_device) {
                  Ok(()) => {
                    let _ = Config::app().emit("music:reload_device", true);
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
  fn download_file(current_loading_id: u64, file_path: &PathBuf, response: Response) {
    let Ok(mut file) = OpenOptions::new().write(true).create(true).open(file_path) else {
      return;
    };

    async_runtime::spawn(async move {
      let mut stream = response.bytes_stream();
      while let Some(chunk_result) = stream.next().await {
        if MUSIC_PLAYER.loading_id.load(Ordering::Acquire) != current_loading_id {
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

            MUSIC_PLAYER
              .downloaded_size
              .fetch_add(chunk.len() as u64, Ordering::AcqRel);
          }
          Err(e) => {
            log_err!("download_file", e, "读取音频流失败");
            break;
          }
        }
      }
    });
  }

  /// ## 加载流式文件
  ///
  /// ### 必选参数
  /// * `current_loading_id` - 当前加载的id, 用于中止无效的加载
  /// * `file_path` - 文件路径
  /// * `file_size` - 文件总大小
  fn load_stream(current_loading_id: u64, file_path: &PathBuf) {
    let Ok(file) = OpenOptions::new().read(true).open(file_path) else {
      return;
    };

    async_runtime::spawn(async move {
      let wait_result = timeout(FILE_TIMEOUT, async {
        loop {
          if MUSIC_PLAYER.loading_id.load(Ordering::Acquire) != current_loading_id {
            break None;
          }

          // 下载足够的数据时再读取
          if MUSIC_PLAYER.downloaded_size.load(Ordering::Acquire) < MIN_READ_SIZE {
            sleep(DOWNLOAD_INTERVAL).await;
            continue;
          }

          break Some(file);
        }
      })
      .await;

      let Ok(result) = wait_result else {
        return;
      };
      let Some(file) = result else {
        return;
      };

      let file_size = MUSIC_PLAYER.audio_size.load(Ordering::Acquire);

      let stream_file = StreamFile::new(file, file_size, &MUSIC_PLAYER.downloaded_size);
      let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

      if let Err(e) = audio.load_from_stream(stream_file, file_size) {
        log_err!("load_stream", e, "加载音频流失败")
      }
    });
  }

  /// ## 平滑音量播放
  ///
  /// 在固定时长内将音量渐变到目标值, 结束后可按需暂停
  ///
  /// ### 必选参数
  /// * `target_volume` - 目标音量, 取值范围为 0 ~ 1
  /// * `pause_after` - 渐变结束后是否暂停
  fn fade_to(target_volume: f32, pause_after: bool) {
    let current_fade_id = MUSIC_PLAYER.fade_id.fetch_add(1, Ordering::AcqRel) + 1;

    async_runtime::spawn(async move {
      let start_volume = if pause_after {
        // 暂停: 从当前音量淡出到 0
        f32::from_bits(MUSIC_PLAYER.audio_volume.load(Ordering::Acquire))
      } else {
        let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());
        audio.set_volume(0.0);
        audio.play();

        // 播放: 从 0 淡入到目标音量
        0.0
      };

      let steps = (FADE_DURATION.as_millis() / FADE_INTERVAL.as_millis()).max(1);
      for step in 1..=steps {
        if MUSIC_PLAYER.fade_id.load(Ordering::Acquire) != current_fade_id {
          return;
        }

        sleep(FADE_INTERVAL).await;

        let volume = start_volume + (target_volume - start_volume) * (step as f32 / steps as f32);

        let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());
        audio.set_volume(volume)
      }

      if pause_after {
        let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());
        audio.pause();
      }
    });
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

impl TryFrom<Device> for DeviceInfo {
  type Error = anyhow::Error;

  fn try_from(device: Device) -> Result<Self, Self::Error> {
    DeviceInfo::try_from(&device)
  }
}

#[tauri::command]
/// ## 获取当前输出设备
pub fn music_player_get_device() -> Result<DeviceInfo, String> {
  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  DeviceInfo::try_from(audio.output_device())
    .log_command("music_player_get_device", "获取当前输出设备失败")
}

#[tauri::command]
/// ## 设置当前输出设备
///
/// ### 必选参数
/// * `id` - 设备id
pub fn music_player_set_device(id: &str) -> Result<(), String> {
  let mut audio = MUSIC_PLAYER
    .audio
    .write()
    .unwrap_or_else(|e| e.into_inner());

  let devices = audio
    .output_devices()
    .log_command("music_player_set_device", "获取输出设备列表失败")?;
  let device = devices
    .into_iter()
    .find(|d| d.id().is_ok_and(|i| i.to_string() == id))
    .ok_or("未获取到输出设备")?;

  audio
    .reload_device(device)
    .log_command("music_player_set_device", "重载播放设备失败")
}

#[tauri::command]
/// ## 获取所有输出设备
pub fn music_player_get_devices() -> Result<Vec<DeviceInfo>, String> {
  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  let devices = audio
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
  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio
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

  let response = Client::new(url, Method::GET)
    .send()
    .await
    .log_command("music_player_load_url", "请求音频资源失败")?;
  let file_size = response.content_length().unwrap_or_default();
  if file_size == 0 {
    return Err("获取音频长度失败".to_string());
  }

  // 重置所有状态
  MUSIC_PLAYER.audio_size.store(file_size, Ordering::Release);
  MUSIC_PLAYER.downloaded_size.store(0, Ordering::Release);

  let file_path = Dir::audio().join(&hash);

  // 检查文件是否存在且为全量数据
  if let Ok(metadata) = metadata(&file_path) {
    if metadata.len() == file_size {
      MUSIC_PLAYER
        .downloaded_size
        .store(file_size, Ordering::Release);

      return music_player_load_file(file_path.to_string_lossy().into_owned());
    }
  }

  let current_loading_id = MUSIC_PLAYER.loading_id.fetch_add(1, Ordering::AcqRel) + 1;
  Player::download_file(current_loading_id, &file_path, response);
  Player::load_stream(current_loading_id, &file_path);

  Ok(())
}

#[tauri::command]
/// ## 监测下载进度
///
/// ### 必选参数
/// * `channel` - 下载进度的通道, 返回值为 0 ~ 1 的进度值
pub fn music_player_monitor_download(channel: Channel<f32>) {
  let current_download_id = MUSIC_PLAYER.download_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = interval(DOWNLOAD_INTERVAL);

    loop {
      interval.tick().await;

      if MUSIC_PLAYER.download_id.load(Ordering::Acquire) != current_download_id {
        break;
      }

      let audio_size = MUSIC_PLAYER.audio_size.load(Ordering::Acquire) as f32;
      let downloaded_size = MUSIC_PLAYER.downloaded_size.load(Ordering::Acquire) as f32;

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
  let current_play_id = MUSIC_PLAYER.play_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = interval(PLAY_INTERVAL);

    // TODO: 即使使用的原子类型判断, 前端页面刷新时旧的循环仍然会执行几次
    // 定义了stop函数,在前端页面卸载时调用也不行, 待优化
    loop {
      interval.tick().await;

      if MUSIC_PLAYER.play_id.load(Ordering::Acquire) != current_play_id {
        break;
      }

      let progress = {
        let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

        match audio.paused() {
          true => continue,
          false => audio.get_pos().as_secs_f32(),
        }
      };

      let _ = channel.send(progress);
    }
  });
}

#[tauri::command]
/// ## 播放
pub fn music_player_play() {
  let audio_volume = MUSIC_PLAYER.audio_volume.load(Ordering::Acquire);

  Player::fade_to(f32::from_bits(audio_volume), false);
}

#[tauri::command]
/// ## 暂停
pub fn music_player_pause() {
  Player::fade_to(0.0, true);
}

#[tauri::command]
/// ## 停止
pub fn music_player_stop() {
  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio.stop()
}

#[tauri::command]
/// ## 跳转播放进度
///
/// ### 必选参数
/// * `pos` - 播放位置, 单位为秒
pub fn music_player_seek(pos: f32) -> Result<(), String> {
  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio
    .try_seek(Duration::from_secs_f32(pos))
    .log_command("music_player_seek", "跳转失败")
}

#[tauri::command]
/// ## 设置音量
///
/// ### 必选参数
/// * `volume` - 音量, 取值范围为 0 ~ 100
pub fn music_player_set_volume(volume: f32) {
  let target_volume = (volume / 100.0).clamp(0.0, 1.0);

  let audio = MUSIC_PLAYER.audio.read().unwrap_or_else(|e| e.into_inner());

  audio.set_volume(target_volume);
  MUSIC_PLAYER
    .audio_volume
    .store(target_volume.to_bits(), Ordering::Release);
}
