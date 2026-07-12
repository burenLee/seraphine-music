use anyhow::anyhow;
use flate2::read::ZlibDecoder;
use std::io::Read;

use crate::utils::crypto::encrypt_md5;

const RANDOM_STRING: &[u8; 36] = b"1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

/// 生成 `len` 长度的随机字符串
pub fn random_string(len: usize) -> String {
  (0..len)
    .map(|_| RANDOM_STRING[rand::random_range(..RANDOM_STRING.len())] as char)
    .collect()
}

/// 计算 mid
///
/// 将 MD5 哈希值转换为十进制大整数字符串
pub fn calculate_mid(data: impl AsRef<[u8]>) -> String {
  let Ok(value) = u128::from_str_radix(&encrypt_md5(data), 16) else {
    // 理论上不会发生，因为 MD5 始终是有效的 128 位(32个十六进制)字符
    return String::new();
  };

  value.to_string()
}

/// 是否合法 hash
pub fn is_valid_hash(hash: &str) -> bool {
  if hash.is_empty() || hash.len() >= 256 {
    return false;
  }

  if hash.contains("..") || hash.starts_with('/') || hash.starts_with('\\') {
    return false;
  }

  hash
    .chars()
    .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

/// 获取合法路径
pub fn get_valid_path(name: &str) -> String {
  name
    .chars()
    .filter(|c| !c.is_control() && !INVALID_CHARS.contains(c))
    .collect()
}

/// 解码 KRC 歌词
pub fn decode_krc_lyric(encoded_data: &[u8]) -> anyhow::Result<String> {
  if encoded_data.len() < 4 {
    return Err(anyhow!("数据长度不足"));
  }

  const KRC_KEY: [u8; 16] = [
    0x40, 0x47, 0x61, 0x77, 0x5E, 0x32, 0x74, 0x47, 0x51, 0x36, 0x31, 0x2D, 0xCE, 0xD2, 0x6E, 0x69,
  ];

  let encrypted_data = &encoded_data[4..];

  let mut decrypted_data = Vec::with_capacity(encrypted_data.len());
  for (i, &byte) in encrypted_data.iter().enumerate() {
    decrypted_data.push(byte ^ KRC_KEY[i % KRC_KEY.len()]);
  }

  let mut decoded_data = String::new();
  let mut decoder = ZlibDecoder::new(&decrypted_data[..]);
  decoder.read_to_string(&mut decoded_data)?;

  Ok(decoded_data)
}
