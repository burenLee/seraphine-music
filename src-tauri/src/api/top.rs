use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{
    config::{DynamicModeConfig, HttpConfig, StaticConfigMode},
    cookie::{HttpCookie, ModeCookies},
    request::HttpRequest,
  },
  utils::{helper::sign_key_params, logger::LogErrExt},
};

#[tauri::command]
/// ## 歌曲推荐
///
/// ### 必选参数
/// * `card_id` - 1: 精选好歌随心听, 2: 经典怀旧金曲, 3: 热门好歌精选, 4: 小众宝藏佳作, 6: vip专属推荐
pub async fn api_top_card(card_id: u8) -> ApiResult<HashMap<String, Value>> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };
  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp_millis();
  let fakem = "60f7ebf1f812edbac3c63a7310001701760f";
  let platform = "android";
  let area_code = "1";

  let params = json!({
    "card_id": card_id,
    "fakem": fakem,
    "platform": platform,
    "area_code": area_code,
  });
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({
    "appid": static_config.appid,
    "clientver": static_config.client_ver,
    "userid": cookies.userid,
    "mid": dynamic_config.mid,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "clienttime": client_time,
    "fakem": fakem,
    "platform": platform,
    "area_code": area_code,
    "uuid": '-',
    "client_playlist": [],
    "u_info": "a0c35cd40af564444b5584c2754dedec"
  });

  HttpRequest::new()
    .url("/singlecardrec.service/v1/single_card_recommend")
    .post()
    .params(params)
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_top_card", "请求失败")
}

#[tauri::command]
/// ## 新碟上架
pub async fn api_top_album() -> ApiResult<HashMap<String, Value>> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let data = json!({
    "apiver": static_config.api_ver,
    "token": cookies.token,
    "withpriv": 1,
  });

  HttpRequest::new()
    .url("/musicadservice/v1/mobile_newalbum_sp")
    .post()
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_top_album", "请求失败")
}

#[tauri::command]
/// ## 推荐歌单
///
/// ### 必选参数
/// * `category_id` - 0：推荐，11292：HI-RES，从 api_playlist_tags 获取（tag_id 即为 category_id）
///
/// ### 可选参数
/// * `module_id` - 模块 id, 默认为 1
/// * `withtag` - 是否返回歌单分类, 0: 不返回, 1: 返回, 默认为 0
/// * `withsong` - 是否返回歌曲列表（不全）, 0: 不返回, 1: 返回, 默认为 0
/// * `sort` - 排序, 默认为 1
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_top_playlist(
  category_id: &str,
  module_id: Option<u64>,
  withtag: Option<u8>,
  withsong: Option<u8>,
  sort: Option<u64>,
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
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp();
  let platform = "android";
  let area_code = "1";

  let data = json!({
    "module_id": module_id.unwrap_or(1),
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
    "appid": static_config.appid,
    "clientver": static_config.client_ver,
    "userid": cookies.userid,
    "mid": dynamic_config.mid,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "clienttime": client_time,
    "platform": platform,
    "req_multi": 1,
    "retrun_min": 5,
    "return_special_falg": 1,
    "special_recommend": {
      "categoryid": category_id,
      "withtag":  withtag.unwrap_or(0),
      "withsong": withsong.unwrap_or(0),
      "sort":  sort.unwrap_or(1),
      "area_code": area_code,
      "ugc": 1,
      "is_selected": 0,
      "withrecommend": 1,
    }
  });

  HttpRequest::new()
    .url("/v2/special_recommend")
    .post()
    .header("x-router", "specialrec.service.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_top_playlist", "请求失败")
}
