use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{
    config::{DynamicModeConfig, HttpConfig, StaticConfigMode},
    request::HttpRequest,
  },
  utils::helper::sign_key_params,
};

use crate::utils::logger::LogErrExt;

#[tauri::command]
/// ## 歌手列表
///
/// ### 可选参数
/// * `area_type` - 0：全部，1：华语，2：欧美，3：日韩，4：其他，5：日本，6：韩国
/// * `musician` - 0：默认，3：音乐人
/// * `sex_type` - 0：全部，1：男，2：女，3：组合
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_artist_list(
  area_type: Option<u8>,
  musician: Option<u8>,
  sex_type: Option<u8>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let params = json!({
    "type": area_type.unwrap_or(0),
    "musician": musician.unwrap_or(0),
    "sextype": sex_type.unwrap_or(0),
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
    "showtype": 1,
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/ocean/v6/singer/list")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_artist_list", "请求失败")
}

#[tauri::command]
/// ## 歌手详情
///
/// ### 必选参数
/// * `id` - 歌手 id
pub async fn api_artist_detail(id: &str) -> ApiResult<HashMap<String, Value>> {
  let data = json!({ "author_id": id });

  HttpRequest::new()
    .url("/kmr/v3/author")
    .post()
    .header("x-router", "openapi.kugou.com")
    .header("kg-tid", "36")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_artist_detail", " 请求失败")
}

#[tauri::command]
/// ## 歌手单曲
///
/// ### 必选参数
/// * `id` - 歌手 id
///
/// ### 可选参数
/// * `sort` - 分类: 1：最热，2：最新
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_artist_audios(
  id: &str,
  sort: Option<u8>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };
  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };

  let client_time = Utc::now().timestamp();

  let data = json!({
    "appid": static_config.appid,
    "clientver": static_config.client_ver,
    "mid": dynamic_config.mid,
    "clienttime": client_time,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "author_id": id,
    "sort": sort.unwrap_or(1),
    "area_code": "all",
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
  });

  HttpRequest::new()
    .base_url("https://openapi.kugou.com")
    .url("/kmr/v1/audio_group/author")
    .post()
    .header("x-router", "openapi.kugou.com")
    .header("kg-tid", "220")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_artist_audios", "请求失败")
}
