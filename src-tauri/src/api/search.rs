use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fmt};

use crate::{
  api::types::ApiResult,
  http::{
    cookie::{HttpCookie, ModeCookies},
    request::HttpRequest,
  },
  utils::logger::LogErrExt,
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
/// ## 搜索
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
    "pagesize": page_size.unwrap_or(10),
    "platform": "AndroidFilter",
  });
  let Value::Object(params) = params else { unreachable!() };

  let search_type = search_type.unwrap_or(SearchType::Song);
  let ver = match search_type {
    SearchType::Song => "v3",
    _ => "v1",
  };

  HttpRequest::new()
    .url(format!("/{ver}/search/{search_type}"))
    .header("x-router", "complexsearch.kugou.com")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_search", "请求失败")
}

#[tauri::command]
/// ## 综合搜索
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
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let params = json!({
    "platform": "AndroidFilter",
    "keyword": keywords,
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
    "cursor": 0,
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .base_url("https://complexsearch.kugou.com")
    .url("/v6/search/complex")
    // .header("cookie", cookies.to_cookies_str())
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_search_complex", "请求失败")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_type_display_variants() {
    assert_eq!(SearchType::Song.to_string(), "song");
    assert_eq!(SearchType::Album.to_string(), "album");
    assert_eq!(SearchType::Author.to_string(), "author");
    assert_eq!(SearchType::Mv.to_string(), "mv");
    assert_eq!(SearchType::Lyric.to_string(), "lyric");
    assert_eq!(SearchType::Special.to_string(), "special");
    assert_eq!(SearchType::Collect.to_string(), "collect");
  }

  #[test]
  fn search_type_serializes_lowercase() {
    assert_eq!(
      serde_json::to_string(&SearchType::Song).unwrap(),
      "\"song\""
    );
    assert_eq!(
      serde_json::to_string(&SearchType::Collect).unwrap(),
      "\"collect\""
    );
  }
}
