use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::libs::ApiResult,
  http::server::{request, RequestOptions},
};

#[tauri::command]
/// ## 专辑音乐列表
///
/// ### 必选参数
/// * `id` - 专辑 id
///
/// ### 可选参数
/// * `is_buy` - 是否购买, 不确定数据类型 暂时不传
/// * `page` - 默认 1
/// * `page_size` - 默认 10
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
    "pagesize": page_size.unwrap_or(10),
  });

  RequestOptions::new()
    .url("/v1/album_audio/lite")
    .post()
    .add_header("x-router", "openapi.kugou.com")
    .add_header("kg-tid", "255")
    .data(data)
    .builder()
    .json()
    .await
}
