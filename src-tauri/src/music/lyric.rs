use anyhow::anyhow;
use base64::{engine::general_purpose::STANDARD, Engine};
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::Read};

use crate::{app::dir::AppDir, utils::logger::LogErrExt, utils::tools::gen_valid_path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lyric {
  pub id: String,
  pub fmt: LyricFormat,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LyricFormat {
  Krc,
  Lrc,
}

impl fmt::Display for LyricFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Krc => write!(f, "krc"),
      Self::Lrc => write!(f, "lrc"),
    }
  }
}

#[tauri::command]
/// ## 获取本地歌词
///
/// ### 必选参数
/// * `id` - 歌词id
/// * `name` - 歌词名称
/// * `fmt` - 歌词类型
pub fn music_lyric_get(id: &str, name: &str, fmt: LyricFormat) -> Option<Lyric> {
  let (id, name) = (gen_valid_path(id), gen_valid_path(name));
  let lyric_path = AppDir::lyric().join(format!("{name} - {id}.{fmt}"));

  if !lyric_path.exists() {
    return None;
  }

  let lyric_txt = fs::read_to_string(&lyric_path)
    .log_warn("music_lyric_get", "读取失败")
    .ok()?;
  let lyric_buf = STANDARD
    .decode(&lyric_txt)
    .log_warn("music_lyric_get", "解码失败")
    .ok()?;
  let lyric_decoded = match fmt {
    LyricFormat::Krc => decode_krc_lyric(&lyric_buf)
      .log_warn("music_lyric_get", "解析失败")
      .ok()?,
    LyricFormat::Lrc => String::from_utf8(lyric_buf)
      .log_warn("music_lyric_get", "解析失败")
      .ok()?,
  };

  Some(Lyric {
    id: id.to_owned(),
    fmt,
    content: lyric_decoded,
  })
}

/// ## 解码歌词内容
///
/// ### 必选参数
/// * `content` - 歌词内容
/// * `fmt` - 歌词类型
pub fn decode_lyric(content: &str, fmt: &LyricFormat) -> anyhow::Result<String> {
  let content_vec = STANDARD.decode(content)?;

  let decoded_content = match fmt {
    LyricFormat::Krc => decode_krc_lyric(&content_vec)?,
    LyricFormat::Lrc => String::from_utf8(content_vec)?,
  };

  Ok(decoded_content)
}

/// ## 解码 KRC 歌词
///
/// ### 必选参数
/// * `content` - krc 歌词内容
pub fn decode_krc_lyric(content: &[u8]) -> anyhow::Result<String> {
  if content.len() < 4 {
    return Err(anyhow!("数据长度不足"));
  }

  const KRC_KEY: [u8; 16] = [
    0x40, 0x47, 0x61, 0x77, 0x5E, 0x32, 0x74, 0x47, 0x51, 0x36, 0x31, 0x2D, 0xCE, 0xD2, 0x6E, 0x69,
  ];

  let encrypted_data = &content[4..];

  let mut decrypted_data = Vec::with_capacity(encrypted_data.len());
  for (i, &byte) in encrypted_data.iter().enumerate() {
    decrypted_data.push(byte ^ KRC_KEY[i % KRC_KEY.len()]);
  }

  let mut decoded_data = String::new();
  let mut decoder = ZlibDecoder::new(&decrypted_data[..]);
  decoder.read_to_string(&mut decoded_data)?;

  Ok(decoded_data)
}

/// ## 保存歌词
///
/// ### 必选参数
/// * `id` - 歌词id
/// * `name` - 歌词名称
/// * `fmt` - 歌词类型
/// * `conetnt` - 歌词内容
pub fn save_lyric(id: &str, name: &str, fmt: &LyricFormat, conetnt: &str) -> anyhow::Result<()> {
  let (id, name) = (gen_valid_path(id), gen_valid_path(name));
  let lyric_path = AppDir::lyric().join(format!("{name} - {id}.{fmt}"));

  fs::write(lyric_path, conetnt)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use flate2::{write::ZlibEncoder, Compression};
  use std::io::Write;

  use super::*;

  /// 与 `decode_krc_lyric` 内部使用一致的 KRC 异或密钥，用于在测试中构造合法 KRC 数据
  const KRC_KEY: [u8; 16] = [
    0x40, 0x47, 0x61, 0x77, 0x5E, 0x32, 0x74, 0x47, 0x51, 0x36, 0x31, 0x2D, 0xCE, 0xD2, 0x6E, 0x69,
  ];

  /// 构造合法的 KRC 编码数据：4 字节头 + (zlib 压缩后与密钥异或)
  fn encode_krc(plain: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plain.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let mut buf = vec![0u8; 4];
    for (i, &byte) in compressed.iter().enumerate() {
      buf.push(byte ^ KRC_KEY[i % KRC_KEY.len()]);
    }

    buf
  }

  #[test]
  fn lyric_format_display() {
    assert_eq!(LyricFormat::Krc.to_string(), "krc");
    assert_eq!(LyricFormat::Lrc.to_string(), "lrc");
  }

  #[test]
  fn decode_lrc_returns_utf8_content() {
    let content = STANDARD.encode("hello lyric");
    let decoded = decode_lyric(&content, &LyricFormat::Lrc).unwrap();

    assert_eq!(decoded, "hello lyric");
  }

  #[test]
  fn decode_krc_roundtrip() {
    let encoded = encode_krc("krc lyric content");
    let decoded = decode_krc_lyric(&encoded).unwrap();

    assert_eq!(decoded, "krc lyric content");
  }

  #[test]
  fn decode_krc_rejects_short_input() {
    let err = decode_krc_lyric(&[0x01, 0x02]).unwrap_err();
    assert_eq!(err.to_string(), "数据长度不足");
  }
}
