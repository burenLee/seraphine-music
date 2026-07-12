use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::lib::ApiResult,
  http::server::{request, RequestOptions},
};

#[tauri::command]
/// 专辑音乐列表
pub async fn api_album_songs(
  id: u64,
  is_buy: Option<&str>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let data = json!({
    "album_id": id,
    "is_buy": is_buy.unwrap_or_default(),
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(30),
  });

  let ops = RequestOptions::new()
    .url("/v1/album_audio/lite")
    .method(Method::POST)
    .add_header("x-router", "openapi.kugou.com")
    .add_header("kg-tid", "255")
    .data(data);

  request(ops).await.map_err(|e| e.to_string())
}
