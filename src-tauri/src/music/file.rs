use lofty::{
  file::{AudioFile, TaggedFileExt},
  picture::Picture,
  probe::Probe,
  tag::Accessor,
};
use serde::Serialize;
use std::{
  fs::{create_dir_all, metadata, read_dir, remove_file, write},
  path::Path,
  process::Command,
};

use crate::system::path::AppPath;

#[derive(Debug, Default, Serialize)]
pub struct MusicDetail {
  path: String,                 // 路径
  cover: Option<String>,        // 封面
  title: String,                // 标题
  artist: Option<String>,       // 歌手
  album: Option<String>,        // 专辑
  genre: Option<String>,        // 流派
  duration: f64,                // 时长
  overall_bitrate: Option<u32>, // 总比特率(kbps)
  audio_bitrate: Option<u32>,   // 音频比特率(kbps)
  sample_rate: Option<u32>,     // 采样率(Hz)
  bit_depth: Option<u8>,        // 比特深度(bits)
  channels: Option<u8>,         // 声道
  format: Option<String>,       // 文件格式
  size: u64,                    // 文件大小
}

#[tauri::command]
/// 获取音频文件详情
pub fn music_file_detail(file_path: &str) -> Result<MusicDetail, String> {
  let path = Path::new(file_path);
  if !path.exists() {
    return Err(String::from("文件不存在"));
  }

  let Ok(metadata) = metadata(file_path) else {
    return Err(String::from("获取文件信息失败"));
  };

  let mut music_info = MusicDetail::default();

  music_info.path = file_path.to_owned();
  music_info.format = path.extension().map(|e| e.to_string_lossy().into_owned());
  music_info.size = metadata.len();

  let probe = Probe::open(file_path).map_err(|e| e.to_string())?;
  let tagged_file = probe.read().map_err(|e| e.to_string())?;

  if let Some(tag) = tagged_file.primary_tag() {
    let file_stem = path
      .file_stem()
      .and_then(|n| n.to_str())
      .ok_or(format!("无法获取文件名: {}", file_path))?;

    let mut title = tag.title().map(|t| t.into_owned());
    let mut artist = tag.artist().map(|t| t.into_owned());

    if title.is_none() || artist.is_none() {
      // 尝试从文件名中获取标题和艺术家
      let file_stem_slice = file_stem.split_once('-');

      if let Some((stem_title, stem_artist)) = file_stem_slice {
        title = Some(stem_title.trim().to_owned());
        artist = Some(stem_artist.trim().to_owned());
      } else {
        title = Some(file_stem.to_owned());
      }
    }

    if title.is_none() {
      return Err(format!("无法获取文件名称"));
    }

    music_info.title = title.unwrap();
    music_info.artist = artist;
    music_info.album = tag.album().map(|a| a.into_owned());

    // 获取并保存封面
    if let Some(picture) = tag.pictures().get(0) {
      let app_path = AppPath::new();
      let cover_dir = app_path.cover_dir();
      music_info.cover = music_file_cover(cover_dir, file_stem, picture);
    }
  }

  let properties = tagged_file.properties();
  music_info.duration = properties.duration().as_secs_f64();
  music_info.overall_bitrate = properties.overall_bitrate();
  music_info.audio_bitrate = properties.audio_bitrate();
  music_info.sample_rate = properties.sample_rate();
  music_info.bit_depth = properties.bit_depth();
  music_info.channels = properties.channels();

  Ok(music_info)
}

/// 获取并保存音频封面
pub fn music_file_cover(cover_dir: &Path, file_stem: &str, pic: &Picture) -> Option<String> {
  if !cover_dir.exists() {
    if let Err(_) = create_dir_all(&cover_dir) {
      return None;
    };
  }

  let Some(cover_ext) = pic.mime_type().and_then(|t| t.ext()) else {
    return None;
  };

  let cover_path = cover_dir.join(format!("{file_stem}.{cover_ext}"));
  if !cover_path.exists() {
    if let Err(_) = write(&cover_path, pic.data()) {
      return None;
    };
  }

  Some(cover_path.to_string_lossy().into_owned())
}

#[tauri::command]
/// 打开文件所在位置
pub fn music_file_open(path: &str) -> Result<(), String> {
  if path.trim().is_empty() {
    return Err(String::from("文件路径不能为空"));
  }

  #[cfg(target_os = "windows")]
  {
    Command::new("explorer")
      .args(["/select,", path])
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .arg("-R")
      .arg(path)
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  #[cfg(target_os = "linux")]
  {
    let parent = Path::new(path)
      .parent()
      .ok_or_else(|| String::from("打开失败"))?;
    Command::new("xdg-open")
      .arg(parent)
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  Ok(())
}

#[tauri::command]
/// 打开目录
pub fn music_dir_open(path: &str) -> Result<(), String> {
  if path.trim().is_empty() {
    return Err(String::from("目录路径不能为空"));
  }

  #[cfg(target_os = "windows")]
  {
    Command::new("explorer")
      .arg(path)
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .arg(path)
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  #[cfg(target_os = "linux")]
  {
    Command::new("xdg-open")
      .arg(path)
      .spawn()
      .map_err(|_| String::from("打开失败"))?;
  };

  Ok(())
}

#[tauri::command]
/// 清理目录下的文件
pub fn music_file_clear(dir_path: &str) -> Result<(), String> {
  if dir_path.trim().is_empty() {
    return Err(String::from("目录路径不能为空"));
  }

  let dir = Path::new(dir_path);
  if !dir.exists() {
    return Err(String::from("目录不存在"));
  }
  if !dir.is_dir() {
    return Err(String::from("不是目录路径"));
  }

  let entries = read_dir(dir).map_err(|_| String::from("读取目录失败"))?;
  for entry in entries {
    let entry = entry.map_err(|_| String::from("读取目录项失败"))?;
    let path = entry.path();
    // 仅删除文件
    if path.is_file() {
      remove_file(&path).map_err(|_| format!("删除文件失败: {}", path.display()))?;
    }
  }

  Ok(())
}
