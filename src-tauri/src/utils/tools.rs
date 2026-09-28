use tauri::Url;

use crate::http::config::KG_BASE_URL;

/// ## 生成随机字符串
///
/// 字符集为 `1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ`
///
/// ### 必选参数
/// * `len` - 字符串长度
pub fn gen_random_string(len: usize) -> String {
  const RANDOM_STRING: &[u8; 36] = b"1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";

  (0..len)
    .map(|_| RANDOM_STRING[rand::random_range(..RANDOM_STRING.len())] as char)
    .collect()
}

/// ## 是否合法 hash
///
/// ### 必选参数
/// * `hash` - 音频 hash
pub fn is_valid_hash(hash: &str) -> bool {
  if hash.is_empty()
    || hash.len() >= 256
    || hash.contains("..")
    || hash.starts_with('/')
    || hash.starts_with('\\')
  {
    return false;
  }

  hash
    .chars()
    .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

/// ## 生成合法 path
///
/// 剔除控制字符与非法字符
///
/// ### 必选参数
/// * `input` - 原始路径字符串
pub fn gen_valid_path(input: &str) -> String {
  const INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

  input
    .chars()
    .filter(|c| !c.is_control() && !INVALID_CHARS.contains(c))
    .collect()
}

/// ## 生成 Url
///
/// 解析失败时返回酷狗的网关地址
///
/// ### 必选参数
/// * `url` - 地址字符串
pub fn gen_url(url: &str) -> Url {
  url
    .parse::<Url>()
    .unwrap_or_else(|_| KG_BASE_URL.parse::<Url>().unwrap())
}

/// ## 生成合法 cookie
///
/// 剔除 cookie 中不允许出现的字符
///
/// ### 必选参数
/// * `input` - 原始字符串
pub fn gen_valid_cookie(input: impl Into<String>) -> String {
  input
    .into()
    .chars()
    .filter(|c| matches!(c, '!' | '#'..='+' | '-'..=':' | '<'..='[' | ']'..='~'))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gen_valid_path_strips_invalid_and_control_chars() {
    assert_eq!(gen_valid_path("a<b>c:d\"e/f\\g|h?i*j"), "abcdefghij");
    assert_eq!(gen_valid_path("hello\nworld"), "helloworld");
    assert_eq!(gen_valid_path("normal"), "normal");
  }

  #[test]
  fn is_valid_hash_accepts_typical_hash() {
    assert!(is_valid_hash("abc123_-.def"));
    assert!(is_valid_hash(&"a".repeat(255)));
  }

  #[test]
  fn is_valid_hash_rejects_invalid_inputs() {
    assert!(!is_valid_hash(""));
    assert!(!is_valid_hash(&"a".repeat(256)));
    assert!(!is_valid_hash("a..b"));
    assert!(!is_valid_hash("/abc"));
    assert!(!is_valid_hash("\\abc"));
    assert!(!is_valid_hash("abc!def"));
  }

  #[test]
  fn gen_valid_cookie_filters_disallowed_chars() {
    assert_eq!(gen_valid_cookie("a b"), "ab");
    assert_eq!(gen_valid_cookie("a,b"), "ab");
    assert_eq!(gen_valid_cookie("a\"b"), "ab");
    assert_eq!(gen_valid_cookie("abc123"), "abc123");
  }
}
