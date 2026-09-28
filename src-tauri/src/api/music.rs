use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{api::types::ApiResult, http::request::HttpRequest};

use crate::utils::logger::LogErrExt;

#[tauri::command]
/// ## 每日推荐
pub async fn api_music_everyday_recommend() -> ApiResult<HashMap<String, Value>> {
  let params = json!({ "platform": "android" });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/everyday_song_recommend")
    .post()
    .header("x-router", "everydayrec.service.kugou.com")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_music_everyday_recommend", "请求失败")
}
