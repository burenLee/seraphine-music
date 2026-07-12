use serde_json::{Map, Value};

use crate::{http::config::HttpConfig, utils::crypto::encrypt_md5};

/// sign 加密
pub fn sign_params(params: &Map<String, Value>, data: Option<&str>) -> String {
  let padding = HttpConfig::get_kg_static_config().params_padding;

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::String(v) => format!("{k}{v}"),
      _ => format!("{k}{}", v.to_string()),
    })
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();
  let data = data.unwrap_or("");

  encrypt_md5(format!("{params_str}{data}{padding}"))
}

/// Web版本 signature 加密
pub fn sign_params_web(params: &Map<String, Value>) -> String {
  let padding = HttpConfig::get_kg_static_config().params_web_padding;

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::String(v) => format!("{k}={v}"),
      _ => format!("{k}={}", v.to_string()),
    })
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();

  encrypt_md5(format!("{padding}{params_str}{padding}"))
}

/// Android版本 signature 加密
pub fn sign_params_android(params: &Map<String, Value>, data: Option<&Value>) -> String {
  let padding = HttpConfig::get_kg_static_config().params_android_padding;

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(k, v)| match v {
      Value::String(v) => format!("{k}={v}"),
      _ => format!("{k}={}", v.to_string()),
    })
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();
  let data_str = match data {
    Some(data) => match data {
      Value::String(s) => s.to_owned(),
      Value::Null => String::new(),
      _ => data.to_string(),
    },
    None => String::new(),
  };

  encrypt_md5(format!("{padding}{params_str}{data_str}{padding}"))
}

/// Register版本 signature 加密
pub fn sign_params_register(params: &Map<String, Value>) -> String {
  let padding = HttpConfig::get_kg_static_config().params_register_padding;

  let mut params_vec: Vec<String> = params
    .iter()
    .map(|(_, v)| match v {
      Value::String(v) => v.to_owned(),
      _ => v.to_string(),
    })
    .collect();
  params_vec.sort_unstable();

  let params_str = params_vec.concat();

  encrypt_md5(format!("{padding}{params_str}{padding}"))
}

/// signKey 加密
pub fn sign_key(hash: &str, mid: &str, userid: Option<&str>, appid: Option<&str>) -> String {
  let kg_params = HttpConfig::get_kg_static_config();
  let padding = kg_params.key_padding;

  let userid = userid.unwrap_or("0");
  let appid = appid.map_or(kg_params.appid.to_string(), |a| a.to_string());

  encrypt_md5(format!("{hash}{padding}{appid}{mid}{userid}"))
}

/// signKey 加密云盘key
pub fn sign_key_cloud(hash: &str, pid: &str) -> String {
  let padding = HttpConfig::get_kg_static_config().key_cloud_padding;

  encrypt_md5(format!("musicclound{hash}{pid}{padding}"))
}

/// signParams 加密
pub fn sign_key_params(data: &str, appid: Option<&str>, client_ver: Option<&str>) -> String {
  let kg_params = HttpConfig::get_kg_static_config();
  let padding = kg_params.key_params_padding;

  let appid = appid.map_or(kg_params.appid.to_string(), |a| a.to_string());
  let client_ver = client_ver.map_or(kg_params.client_ver.to_string(), |c| c.to_string());

  encrypt_md5(format!("{appid}{padding}{client_ver}{data}"))
}
