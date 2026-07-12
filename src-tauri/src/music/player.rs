use anyhow::Result;
use rodio::{
  cpal::{default_host, traits::HostTrait},
  Device, DeviceTrait,
};
use serde::Serialize;
use std::{
  fs::{metadata, OpenOptions},
  io::Write,
  path::PathBuf,
  sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, RwLock,
  },
  time::Duration,
};
use tauri::{async_runtime, ipc::Channel, AppHandle, Emitter, State};
use tauri_plugin_http::reqwest::Response;
use tokio::time;
use tokio_stream::StreamExt;

use crate::{
  http::client::HttpRequest,
  music::{audio::Audio, stream::StreamFile},
  system::path::AppPath,
  utils::tools::is_valid_hash,
};

// 获取文件句柄超时
const FILE_TIMEOUT: Duration = Duration::from_millis(5000);
// 监测设备的间隔
const DEVICE_INTERVAL: Duration = Duration::from_millis(1000);
// 播放进度的获取间隔
const PLAY_INTERVAL: Duration = Duration::from_millis(16);
// 下载进度的获取间隔
const DOWNLOAD_INTERVAL: Duration = Duration::from_millis(100);
// 最小读取文件大小 128 kb
const MIN_READ_SIZE: u64 = 1024 * 128;

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
  id: String,
  name: String,
}

impl DeviceInfo {
  fn from_device(device: &Device) -> Option<Self> {
    let Ok(id) = device.id() else {
      return None;
    };
    let Ok(description) = device.description() else {
      return None;
    };

    let name = description.name();
    let driver = description.driver().unwrap_or_default();

    Some(DeviceInfo {
      id: id.to_string(),
      name: format!("{name}({driver})"),
    })
  }
}

pub struct Player {
  app_path: AppPath,
  audio: Arc<RwLock<Audio>>,           // 音频实例
  audio_size: Arc<AtomicU64>,          // 音频文件大小
  play_channel_id: Arc<AtomicU64>,     // 播放进度的id
  download_channel_id: Arc<AtomicU64>, // 下载进度的id
  loading_id: Arc<AtomicU64>,          // 加载url的id
  is_downloading: Arc<AtomicBool>,     // 是否正在下载
  downloaded_size: Arc<AtomicU64>,     // 已下载的文件大小
}

impl Player {
  pub fn new(app_handle: &AppHandle) -> Result<Player> {
    let audio = Audio::new()?;

    let player = Self {
      app_path: AppPath::new(),
      audio: Arc::new(RwLock::new(audio)),
      audio_size: Arc::new(AtomicU64::new(0)),
      play_channel_id: Arc::new(AtomicU64::new(0)),
      download_channel_id: Arc::new(AtomicU64::new(0)),
      loading_id: Arc::new(AtomicU64::new(0)),
      is_downloading: Arc::new(AtomicBool::new(false)),
      downloaded_size: Arc::new(AtomicU64::new(0)),
    };

    player.monitor_device(app_handle);

    Ok(player)
  }

  /// 监测设备变动
  fn monitor_device(&self, app_handle: &AppHandle) {
    let app = app_handle.clone();
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
                    if let Err(e) = app.emit("reload_device", true) {
                      eprintln!("{e}");
                    };
                  }
                  Err(e) => eprintln!("{e}"),
                }
              }
            }

            old_did = new_did;
          }
        }
      }
    });
  }

  // 下载文件
  pub fn download_file(
    &self,
    current_loading_id: u64,
    file_path: &PathBuf,
    response: Response,
  ) -> Result<(), String> {
    let loading_id = self.loading_id.clone();
    let is_downloading = self.is_downloading.clone();
    let downloaded_size = self.downloaded_size.clone();

    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(file_path)
      .map_err(|e| e.to_string())?;

    async_runtime::spawn(async move {
      is_downloading.store(true, Ordering::Release);

      let mut stream = response.bytes_stream();
      while let Some(chunk_result) = stream.next().await {
        if loading_id.load(Ordering::Acquire) != current_loading_id {
          eprintln!("下载被取消: {}", current_loading_id);
          break;
        }

        match chunk_result {
          Ok(chunk) => {
            if let Err(e) = file.write_all(&chunk) {
              eprintln!("写入失败: {}", e);
              break;
            }

            if let Err(e) = file.flush() {
              eprintln!("flush 失败: {}", e);
              break;
            }

            downloaded_size.fetch_add(chunk.len() as u64, Ordering::AcqRel);
          }
          Err(e) => {
            eprintln!("读取流失败: {}", e);
            break;
          }
        }
      }

      is_downloading.store(false, Ordering::Release);
    });

    Ok(())
  }

  // 加载流式文件
  pub fn load_stream(&self, current_loading_id: u64, file_path: &PathBuf, file_size: u64) {
    let file_path = file_path.clone();
    let audio = self.audio.clone();
    let loading_id = self.loading_id.clone();
    let downloaded_size = self.downloaded_size.clone();

    async_runtime::spawn(async move {
      let wait_result = time::timeout(FILE_TIMEOUT, async {
        loop {
          if loading_id.load(Ordering::Acquire) != current_loading_id {
            eprintln!("加载被取消: {}", current_loading_id);
            break None;
          }

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
            eprintln!("加载前已被取消");
            return;
          }

          if let Ok(mut audio_writer) = audio.write() {
            let stream_file = StreamFile::new(file, file_size, downloaded_size);
            if let Err(e) = audio_writer.load_from_stream(stream_file, file_size) {
              eprintln!("加载音频流失败: {}", e);
            }
          }
        }
        Ok(None) => {
          eprintln!("等待文件取消")
        }
        Err(_) => {
          eprintln!("等待文件超时");
        }
      }
    });
  }
}

#[tauri::command]
/// 获取当前输入设备
pub fn music_player_get_device(state: State<Player>) -> Result<Option<DeviceInfo>, String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;
  let Some(device) = audio_reader.current_device() else {
    return Ok(None);
  };

  Ok(DeviceInfo::from_device(device))
}

#[tauri::command]
/// 设置当前输入设备
///
/// ### 必选参数
/// * `id` - 设备id
pub fn music_player_set_device(state: State<Player>, id: &str) -> Result<(), String> {
  let mut audio_writer = state.audio.write().map_err(|e| e.to_string())?;

  let devices = audio_writer.all_devices();
  let Some(device) = devices
    .into_iter()
    .find(|d| d.id().is_ok_and(|i| i.to_string() == id))
  else {
    return Err(String::from("未找到输出设备"));
  };

  audio_writer
    .reload_device(device)
    .map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取所有输出设备
pub fn music_player_get_devices(state: State<Player>) -> Result<Vec<DeviceInfo>, String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;

  let device_info_vec = audio_reader
    .all_devices()
    .iter()
    .filter_map(|d| DeviceInfo::from_device(d))
    .collect();

  Ok(device_info_vec)
}

#[tauri::command]
pub fn music_player_load_file(state: State<Player>, path: String) -> Result<(), String> {
  let mut audio_writer = state.audio.write().map_err(|e| e.to_string())?;

  audio_writer
    .load_from_file(&path)
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn music_player_load_url(
  state: State<'_, Player>,
  path: String,
  hash: String,
) -> Result<(), String> {
  // 验证 hash 参数的合法性
  if !is_valid_hash(&hash) {
    return Err("无效的哈希值".to_string());
  }

  let response = HttpRequest::get(path).await.map_err(|e| e.to_string())?;
  let file_size = response
    .content_length()
    .ok_or_else(|| "无法获取文件大小".to_string())?;

  if file_size == 0 {
    return Err("文件大小为0".to_string());
  }

  // 保存id快照
  let current_loading_id = state.loading_id.fetch_add(1, Ordering::AcqRel) + 1;
  // 重置所有状态
  state.is_downloading.store(true, Ordering::Release);
  state.downloaded_size.store(0, Ordering::Release);
  state.audio_size.store(file_size, Ordering::Release);

  let file_path = state.app_path.temp_dir().join(&hash);

  // 检查文件是否存在且全量数据
  if let Ok(metadata) = metadata(&file_path) {
    if metadata.len() == file_size {
      state.is_downloading.store(false, Ordering::Release);
      state.downloaded_size.store(file_size, Ordering::Release);
      music_player_load_file(state, file_path.to_string_lossy().into_owned())?;

      return Ok(());
    }
  }

  state.download_file(current_loading_id, &file_path, response)?;
  state.load_stream(current_loading_id, &file_path, file_size);

  Ok(())
}

#[tauri::command]
/// 监测下载进度
pub fn music_player_monitor_download(state: State<Player>, channel: Channel<f32>) {
  let download_channel_id = state.download_channel_id.clone();
  let downloaded_size = state.downloaded_size.clone();
  let audio_size = state.audio_size.clone();

  let current_channel_id = download_channel_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = time::interval(PLAY_INTERVAL);

    loop {
      if current_channel_id != download_channel_id.load(Ordering::Acquire) {
        eprintln!("下载进度事件取消 {}", current_channel_id);
        break;
      }

      interval.tick().await;

      let downloaded_size = downloaded_size.load(Ordering::Acquire) as f32;
      let audio_size = audio_size.load(Ordering::Acquire) as f32;

      if downloaded_size == 0.0 || audio_size == 0.0 {
        continue;
      }

      if let Err(e) = channel.send(downloaded_size / audio_size) {
        eprintln!("发送进度事件失败: {}", e);
        break;
      }

      time::sleep(DOWNLOAD_INTERVAL).await;
    }
  });
}

#[tauri::command]
/// 监测播放进度
pub fn music_player_monitor_play(state: State<Player>, channel: Channel<f32>) {
  let audio = state.audio.clone();
  let play_channel_id = state.play_channel_id.clone();
  let current_channel_id = play_channel_id.fetch_add(1, Ordering::AcqRel) + 1;

  async_runtime::spawn(async move {
    let mut interval = time::interval(PLAY_INTERVAL);

    // TODO: 即使使用的原子类型判断, 前端页面刷新时旧的循环仍然会执行几次
    // 定义了stop函数,在前端页面卸载时调用也不行, 待优化
    loop {
      if current_channel_id != play_channel_id.load(Ordering::Acquire) {
        eprintln!("播放进度事件取消 {}", current_channel_id);
        break;
      }

      interval.tick().await;

      let progress = match audio.read() {
        Ok(audio_reader) => {
          if !audio_reader.paused() {
            audio_reader.get_pos().as_secs_f32()
          } else {
            continue;
          }
        }
        Err(_) => continue,
      };

      if let Err(e) = channel.send(progress) {
        eprintln!("发送进度事件失败: {}", e);
        break;
      }
    }
  });
}

#[tauri::command]
pub fn music_player_play(state: State<Player>) -> Result<(), String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;

  Ok(audio_reader.play())
}

#[tauri::command]
pub fn music_player_pause(state: State<Player>) -> Result<(), String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;

  Ok(audio_reader.pause())
}

#[tauri::command]
pub fn music_player_stop(state: State<Player>) -> Result<(), String> {
  let mut audio_writer = state.audio.write().map_err(|e| e.to_string())?;

  Ok(audio_writer.stop())
}

#[tauri::command]
pub fn music_player_seek(state: State<Player>, pos: f32) -> Result<(), String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;

  audio_reader
    .try_seek(Duration::from_secs_f32(pos))
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn music_player_set_volume(state: State<Player>, volume: f32) -> Result<(), String> {
  let audio_reader = state.audio.read().map_err(|e| e.to_string())?;

  Ok(audio_reader.set_volume(volume / 100.0))
}
