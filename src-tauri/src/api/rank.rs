use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{api::types::ApiResult, http::request::HttpRequest, utils::logger::LogErrExt};

#[tauri::command]
/// ## 排行榜列表
///
/// ### 可选参数
/// * `withsong` - 是否返回歌曲列表（不全）, 0: 不返回, 1: 返回
pub async fn api_rank_list(withsong: Option<u8>) -> ApiResult<HashMap<String, Value>> {
  let params = json!({ "withsong": withsong.unwrap_or(0), "plat": 2, "parentid": 0 });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/ocean/v6/rank/list")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_rank_list", "请求失败")
}

#[tauri::command]
/// ## 排行榜推荐列表
pub async fn api_rank_top() -> ApiResult<HashMap<String, Value>> {
  HttpRequest::new()
    .url("/mobileservice/api/v5/rank/rec_rank_list")
    .builder()
    .json()
    .await
    .log_command("api_rank_top", "请求失败")
}

#[tauri::command]
/// ## 排行榜歌曲列表
///
/// ### 必选参数
/// * `rank_id` - 排行榜id
///
/// ### 可选参数
/// * `rank_cid` - 往期列表id, `/rank/vol` 的 `volid` 字段
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_rank_audio(
  rank_id: u64,
  rank_cid: Option<u64>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let data = json!({
    "rank_id": rank_id,
    "rank_cid": rank_cid.unwrap_or(0),
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
    "show_portrait_mv": 1,
    "show_type_total": 1,
    "filter_original_remarks": 1,
    "area_code": 1,
    "type": 1,
  });

  HttpRequest::new()
    .url("/openapi/kmr/v2/rank/audio")
    .post()
    .header("kg-tid", "369")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_rank_audio", "请求失败")
}
