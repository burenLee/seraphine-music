use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::lib::ApiResult,
  http::{
    config::HttpConfig,
    server::{request, RequestOptions},
  },
  utils::helper::sign_key_params,
};

#[tauri::command]
/// 歌手列表
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

  let opts = RequestOptions::new()
    .url("/ocean/v6/singer/list")
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 歌手单曲
///
/// ### 必选参数
/// * `id` - 歌手 id
///
/// ### 可选参数
/// * `sort` - 分类: 1：最热，2：最新
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_artist_audios(
  id: u64,
  sort: Option<u8>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();
  let kg_static_config = HttpConfig::get_kg_static_config();

  let mid = kg_dynamic_config.mid;
  let client_time = Utc::now().timestamp();

  let data = json!({
    "appid": kg_static_config.appid,
    "clientver": kg_static_config.client_ver,
    "mid": mid,
    "clienttime": client_time,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "author_id": id,
    "sort": sort.unwrap_or(1),
    "area_code": "all",
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
  });

  let opts = RequestOptions::new()
    .base_url("https://openapi.kugou.com")
    .url("/kmr/v1/audio_group/author")
    .method(Method::POST)
    .add_header("x-router", "openapi.kugou.com")
    .add_header("kg-tid", "220")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}
