use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
  system::path::AppPath,
  utils::tools::{decode_krc_lyric, get_valid_path},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lyric {
  pub id: String,
  pub fmt: LyricFormat,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LyricFormat {
  Krc,
  Lrc,
}

impl LyricFormat {
  pub fn as_ext(&self) -> &'static str {
    match self {
      Self::Krc => "krc",
      Self::Lrc => "lrc",
    }
  }
}

#[tauri::command]
/// 获取本地歌词
///
/// ### 必选参数
/// * `name` - 歌词名称
/// * `id` - 歌词id
pub fn music_lyric_get(name: &str, id: &str, fmt: LyricFormat) -> Option<Lyric> {
  let (name, id) = (get_valid_path(name), get_valid_path(id));

  let lyric_path = AppPath::new()
    .lyric_dir()
    .join(format!("{name} - {id}.{}", fmt.as_ext()));

  let Ok(lyric_txt) = fs::read_to_string(&lyric_path) else {
    return None;
  };

  let Ok(lyric_buf) = STANDARD.decode(&lyric_txt) else {
    return None;
  };

  let lyric_decoded = match fmt {
    LyricFormat::Krc => decode_krc_lyric(&lyric_buf).ok(),
    LyricFormat::Lrc => String::from_utf8(lyric_buf).ok(),
  };

  Some(Lyric {
    id: id.to_owned(),
    fmt,
    content: lyric_decoded.unwrap_or_default(),
  })
}

pub fn music_lyric_save(
  name: &str,
  id: &str,
  fmt: &LyricFormat,
  conetnt: &str,
) -> Result<(), String> {
  let (name, id) = (get_valid_path(name), get_valid_path(id));

  let lyric_path = AppPath::new()
    .lyric_dir()
    .join(format!("{name} - {id}.{}", fmt.as_ext()));

  if let Err(e) = fs::write(&lyric_path, &conetnt) {
    return Err(format!("歌词文件保存失败: {e}"));
  };

  Ok(())
}
