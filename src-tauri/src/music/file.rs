use lofty::{
  file::{AudioFile, TaggedFileExt},
  picture::Picture,
  probe::Probe,
  tag::Accessor,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{app::dir::AppDir, log_err, utils::logger::LogErrExt};

#[derive(Debug, Default, Serialize, Deserialize)]
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
/// ## 获取音频文件详情
///
/// ### 必选参数
/// * `path` - 音频文件路径
pub fn music_file_detail(path: &str) -> Result<MusicDetail, String> {
  let file_path = PathBuf::from(path);
  if !file_path.exists() {
    return Err("文件不存在".to_string());
  }
  if !file_path.is_file() {
    return Err("非文件路径".to_string());
  }

  let mut music_info = MusicDetail::default();

  music_info.path = path.to_owned();
  music_info.format = file_path
    .extension()
    .map(|e| e.to_string_lossy().into_owned());

  let metadata = fs::metadata(path).log_command("music_file_detail", "读取文件元数据失败")?;
  music_info.size = metadata.len();

  let probe = Probe::open(path).log_command("music_file_detail", "打开音频文件失败")?;
  let tagged_file = probe
    .read()
    .log_command("music_file_detail", "解析音频文件失败")?;

  if let Some(tag) = tagged_file.primary_tag() {
    let file_stem = file_path
      .file_stem()
      .map(|n| n.to_string_lossy())
      .ok_or("无法获取文件名称")?;

    music_info.title = tag.title().unwrap_or(file_stem.clone()).into_owned();
    music_info.artist = tag.artist().map(|a| a.into_owned());
    music_info.album = tag.album().map(|a| a.into_owned());

    // 获取并保存封面
    if let Some(picture) = tag.pictures().get(0) {
      music_info.cover = music_file_cover(&file_stem, picture);
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

/// ## 获取并保存音频封面
///
/// ### 必选参数
/// * `file_stem` - 音频文件名称(不含扩展名), 用于封面命名
/// * `pic` - 音频的封面图片
pub fn music_file_cover(file_stem: &str, pic: &Picture) -> Option<String> {
  let cover_ext = pic.mime_type().and_then(|m| m.ext())?;
  let cover_path = AppDir::cover().join(format!("{file_stem}.{cover_ext}"));

  if !cover_path.exists() {
    if let Err(e) = fs::write(&cover_path, pic.data()) {
      log_err!("music_file_cover", e, "写入失败");
      return None;
    }
  }

  Some(cover_path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn music_detail_default() {
    let detail = MusicDetail::default();

    assert_eq!(detail.path, "");
    assert_eq!(detail.cover, None);
    assert_eq!(detail.title, "");
    assert_eq!(detail.artist, None);
    assert_eq!(detail.album, None);
    assert_eq!(detail.duration, 0.0);
    assert_eq!(detail.size, 0);
  }

  #[test]
  fn music_file_detail_nonexistent_returns_err() {
    let result = music_file_detail("/nonexistent/file.mp3");

    assert_eq!(result.unwrap_err(), "文件不存在");
  }

  #[test]
  fn music_file_detail_directory_returns_err() {
    let temp = tempfile::tempdir().unwrap();

    let result = music_file_detail(temp.path().to_string_lossy().as_ref());

    assert_eq!(result.unwrap_err(), "非文件路径");
  }
}
