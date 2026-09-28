use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::libs::ApiResult,
  http::{config::HttpConfig, mode::HttpMode, request::HttpRequest},
  utils::helper::sign_key_params,
};

#[tauri::command]
/// ## 音乐相关信息
///
/// ### 必选参数
/// * `hashs` - 歌曲 hash 列表
pub async fn api_audio_info(hashs: Vec<&str>) -> ApiResult<HashMap<String, Value>> {
  let http_mode = HttpMode::get_mode();
  let kg_static_config = HttpConfig::get_kg_static_config(&http_mode);
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config(&http_mode);

  let client_time = Utc::now().timestamp_millis();
  let data = hashs
    .iter()
    .map(|h| json!({ "hash": h, "audio_id": 0 }))
    .collect::<Vec<_>>();

  let data = json!({
    "appid": kg_static_config.appid,
    "clienttime": client_time.clone(),
    "clientver": kg_static_config.client_ver,
    "data": data,
    "dfid": kg_dynamic_config.cookies.dfid,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "mid": kg_dynamic_config.mid,
    "token": kg_dynamic_config.cookies.token,
    "userid": kg_dynamic_config.cookies.userid,
  });

  HttpRequest::new()
    .base_url("http://kmr.service.kugou.com")
    .url("/v1/audio/audio")
    .post()
    .header("x-router", "kmr.service.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
}
