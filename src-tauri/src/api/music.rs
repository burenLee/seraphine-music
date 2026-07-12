use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::lib::ApiResult,
  http::server::{request, RequestOptions},
};

#[tauri::command]
pub async fn api_music_everyday() -> ApiResult<HashMap<String, Value>> {
  let params = json!({ "platform": "android" });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .url("/everyday_song_recommend")
    .method(Method::POST)
    .add_header("x-router", "everydayrec.service.kugou.com")
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}
