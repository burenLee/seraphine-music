use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{config::HttpConfig, mode::HttpMode, request::HttpRequest},
};

#[derive(Debug, Serialize)]
struct MusicInfo {
  #[serde(rename = "type")]
  music_type: String,
  page_id: u64,
  hash: String,
  album_id: u64,
}

#[tauri::command]
/// ## 歌曲信息
///
/// ### 必选参数
/// * `hashes` - 音频的hash列表
pub async fn api_privilege_lite(hashes: Vec<&str>) -> ApiResult<HashMap<String, Value>> {
  let http_mode = HttpMode::get_mode();
  let kg_static_config = HttpConfig::get_kg_static_config(&http_mode);

  let music_list: Vec<MusicInfo> = hashes
    .iter()
    .map(|h| MusicInfo {
      music_type: "audio".to_string(),
      page_id: 0,
      hash: h.to_string(),
      album_id: 0,
    })
    .collect();

  let data = json!({
    "appid": kg_static_config.appid,
    "area_code": 1,
    "behavior": "play",
    "clientver": kg_static_config.client_ver,
    "need_hash_offset": 1,
    "relate": 1,
    "support_verify": 1,
    "resource": music_list,
    "qualities": vec!["128", "320", "flac", "high", "viper_atmos", "viper_tape", "viper_clear"],
  });

  HttpRequest::new()
    .url("/v2/get_res_privilege/lite")
    .post()
    .header("x-router", "media.store.kugou.com")
    .header("content-type", "application/json")
    .data(data)
    .builder()
    .json()
    .await
}
