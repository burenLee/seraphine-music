use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fmt};

use crate::{
  api::lib::ApiResult,
  http::{
    config::HttpConfig,
    lib::KgCookies,
    server::{request, RequestOptions},
  },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchType {
  Song,
  Album,
  Author,
  Mv,
  Lyric,
  Special,
  Collect,
}

impl fmt::Display for SearchType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      SearchType::Special => "special",
      SearchType::Lyric => "lyric",
      SearchType::Song => "song",
      SearchType::Album => "album",
      SearchType::Author => "author",
      SearchType::Mv => "mv",
      SearchType::Collect => "collect",
    };
    f.write_str(s)
  }
}

#[tauri::command]
/// 搜索
///
/// ### 必选参数
/// * `keywords` - 关键词
///
/// ### 可选参数
/// * `search_type` - 搜索类型, 默认为单曲, special:歌单, lyric:歌词, song:单曲, album:专辑, author:歌手, mv:mv
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_search(
  keywords: &str,
  search_type: Option<SearchType>,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let params = json!({
    "albumhide": 0,
    "iscorrection": 1,
    "keyword": keywords,
    "nocollect": 0,
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(30),
    "platform": "AndroidFilter",
  });
  let Value::Object(params) = params else { unreachable!() };

  let search_type = search_type.unwrap_or(SearchType::Song);
  let ver = match search_type {
    SearchType::Song => "v3",
    _ => "v1",
  };

  let opts = RequestOptions::new()
    .url(format!("/{ver}/search/{search_type}"))
    .add_header("x-router", "complexsearch.kugou.com")
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 综合搜索
///
/// ### 必选参数
/// * `keywords` - 关键词
///
/// ### 可选参数
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_search_complex(
  keywords: &str,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let params = json!({
    "platform": "AndroidFilter",
    "keyword": keywords,
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(30),
    "cursor": 0,
  });
  let Value::Object(params) = params else { unreachable!() };

  let KgCookies {
    dfid,
    userid,
    token,
    t1,
    vip_type,
    vip_token,
  } = HttpConfig::get_kg_dynamic_config().cookies;

  let opts = RequestOptions::new()
    .base_url("https://complexsearch.kugou.com")
    .url("/v6/search/complex")
    .add_header("cookie", format!("dfid={dfid}; userid={userid}; token={token}; t1={t1}; vip_type={vip_type}; vip_token={vip_token}"))
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}
