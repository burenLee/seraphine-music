use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri::http::Method;

use crate::{
  api::lib::ApiResult,
  http::{
    config::HttpConfig,
    server::{request, RequestOptions},
  },
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
/// 获取歌曲信息
///
/// ### 必选参数
/// * `hashes` - 音频的hash列表
pub async fn api_privilege_lite(hashes: Vec<&str>) -> ApiResult<HashMap<String, Value>> {
  let kg_static_config = HttpConfig::get_kg_static_config();

  let music_list: Vec<MusicInfo> = hashes
    .iter()
    .map(|h| MusicInfo {
      music_type: String::from("audio"),
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

  let opts = RequestOptions::new()
    .url("/v2/get_res_privilege/lite")
    .method(Method::POST)
    .add_header("x-router", "media.store.kugou.com")
    .add_header("content-type", "application/json")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}
