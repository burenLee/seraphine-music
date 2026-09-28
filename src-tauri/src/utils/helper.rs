use serde_json::{Map, Value};

use crate::{
  http::config::{HttpConfig, KgStaticConfig, StaticConfigMode},
  utils::crypto::encrypt_md5,
};

/// ## signature 加密
///
/// ### 必选参数
/// * `params` - 请求参数
///
/// ### 可选参数
/// * `data` - 请求体
pub fn sign_params(params: &Map<String, Value>, data: Option<&str>) -> String {
  let &KgStaticConfig { params_padding, .. } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::Null => format!("{k}{}", String::new()),
      Value::String(v) => format!("{k}{v}"),
      _ => format!("{k}{}", v.to_string()),
    }) // keyvalue 格式
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();
  let data_str = data.unwrap_or("");

  encrypt_md5(format!("{params_str}{data_str}{params_padding}",))
}

/// ## signature 加密 Web 版本
///
/// ### 必选参数
/// * `params` - 请求参数
///
/// ### 可选参数
/// * `data` - 请求体
pub fn sign_params_web(params: &Map<String, Value>, data: Option<&str>) -> String {
  let &KgStaticConfig {
    params_web_padding, ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::Null => format!("{k}={}", String::new()),
      Value::String(v) => format!("{k}={v}"),
      _ => format!("{k}={}", v.to_string()),
    }) // key=value 格式
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();
  let data_str = data.unwrap_or("");

  encrypt_md5(format!(
    "{params_web_padding}{params_str}{data_str}{params_web_padding}"
  ))
}

/// ## signature 加密 Android 版本
///
/// ### 必选参数
/// * `params` - 请求参数
///
/// ### 可选参数
/// * `data` - 请求体
pub fn sign_params_android(params: &Map<String, Value>, data: Option<&str>) -> String {
  let &KgStaticConfig {
    params_android_padding,
    ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::Null => format!("{k}={}", String::new()),
      Value::String(v) => format!("{k}={v}"),
      _ => format!("{k}={}", v.to_string()),
    }) // key=value 格式
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();
  let data_str = data.unwrap_or("");

  encrypt_md5(format!(
    "{params_android_padding}{params_str}{data_str}{params_android_padding}"
  ))
}

/// ## signature 加密 Register 版本
///
/// ### 必选参数
/// * `params` - 请求参数
pub fn sign_params_register(params: &Map<String, Value>) -> String {
  let &KgStaticConfig {
    params_register_padding,
    ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(_, v)| match v {
      Value::Null => String::new(),
      Value::String(v) => v.to_owned(),
      _ => v.to_string(),
    }) // 只取value, 忽略key
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();

  encrypt_md5(format!(
    "{params_register_padding}{params_str}{params_register_padding}"
  ))
}

/// ## signKey 加密
///
/// ### 必选参数
/// * `hash` - 音频 hash
/// * `mid` - 设备 mid
///
/// ### 可选参数
/// * `userid` - 用户 id, 默认为 `0`
/// * `appid` - 应用 id, 默认为当前模式的 appid
pub fn sign_key(hash: &str, mid: &str, userid: Option<&str>, appid: Option<&str>) -> String {
  let &KgStaticConfig {
    appid: default_appid,
    key_padding,
    ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let userid = userid.unwrap_or("0");
  let default_appid = default_appid.to_string();
  let appid = appid.unwrap_or(&default_appid);

  encrypt_md5(format!("{hash}{key_padding}{appid}{mid}{userid}"))
}

/// ## signKey 加密 云盘
///
/// ### 必选参数
/// * `hash` - 音频 hash
/// * `pid` - 平台 id
pub fn sign_key_cloud(hash: &str, pid: &str) -> String {
  let &KgStaticConfig {
    key_cloud_padding, ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  encrypt_md5(format!("musicclound{hash}{pid}{key_cloud_padding}"))
}

/// ## signKey 加密 params
///
/// ### 必选参数
/// * `data` - 参与加密的数据, 一般为时间戳
///
/// ### 可选参数
/// * `appid` - 应用 id, 默认为当前模式的 appid
/// * `client_ver` - 客户端版本, 默认为当前模式的 client_ver
pub fn sign_key_params(data: &str, appid: Option<&str>, client_ver: Option<&str>) -> String {
  let &KgStaticConfig {
    appid: default_appid,
    client_ver: default_client_ver,
    key_params_padding,
    ..
  } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let default_appid = default_appid.to_string();
  let appid = appid.unwrap_or(&default_appid);
  let default_client_ver = default_client_ver.to_string();
  let client_ver = client_ver.unwrap_or(&default_client_ver);

  encrypt_md5(format!("{appid}{key_params_padding}{client_ver}{data}"))
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use super::*;

  /// md5 加密结果为 32 位 16 进制字符串
  fn is_md5_hex(s: &str) -> bool {
    s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
  }

  #[test]
  fn sign_key_is_deterministic() {
    let a = sign_key("hash", "mid", None, None);
    let b = sign_key("hash", "mid", None, None);

    assert_eq!(a, b);
    assert!(is_md5_hex(&a));
  }

  #[test]
  fn sign_params_is_order_independent() {
    let mut p1 = Map::new();
    p1.insert("a".to_string(), json!(1));
    p1.insert("b".to_string(), json!(2));

    let mut p2 = Map::new();
    p2.insert("b".to_string(), json!(2));
    p2.insert("a".to_string(), json!(1));

    assert_eq!(sign_params(&p1, None), sign_params(&p2, None));
  }

  #[test]
  fn sign_params_web_is_order_independent() {
    let mut p1 = Map::new();
    p1.insert("a".to_string(), json!("1"));
    p1.insert("b".to_string(), json!("2"));

    let mut p2 = Map::new();
    p2.insert("b".to_string(), json!("2"));
    p2.insert("a".to_string(), json!("1"));

    assert_eq!(sign_params_web(&p1, None), sign_params_web(&p2, None));
  }

  #[test]
  fn sign_params_register_only_uses_values() {
    // register 版本只取 value 忽略 key, 交换 key 不影响结果
    let mut p1 = Map::new();
    p1.insert("k1".to_string(), json!("v1"));
    p1.insert("k2".to_string(), json!("v2"));

    let mut p2 = Map::new();
    p2.insert("k2".to_string(), json!("v1"));
    p2.insert("k1".to_string(), json!("v2"));

    assert_eq!(sign_params_register(&p1), sign_params_register(&p2));
  }

  #[test]
  fn sign_key_cloud_is_deterministic() {
    let a = sign_key_cloud("hash", "pid");

    assert_eq!(a, sign_key_cloud("hash", "pid"));
    assert!(is_md5_hex(&a));
  }

  #[test]
  fn sign_key_params_is_deterministic() {
    let a = sign_key_params("123", None, None);

    assert_eq!(a, sign_key_params("123", None, None));
    assert!(is_md5_hex(&a));
  }
}
