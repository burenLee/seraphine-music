use chrono::Local;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{
    cookie::{HttpCookie, ModeCookies},
    request::HttpRequest,
  },
  utils::logger::LogErrExt,
};

#[tauri::command]
/// ## 已领取 VIP 状态
pub async fn api_youth_union_vip() -> ApiResult<HashMap<String, Value>> {
  let params = json!({
    "busi_type": "concept",
    "opt_product_types": "dvip,qvip",
    "product_type": "svip"
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .base_url("https://kugouvip.kugou.com")
    .url("/v1/get_union_vip")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_youth_union_vip", "请求失败")
}

#[tauri::command]
/// ## 领取 VIP
///
/// ### 可选参数
/// * `receive_day` - 领取 VIP 日期, 格式为 2026-01-30, 默认为当天
pub async fn api_youth_day_vip(receive_day: Option<String>) -> ApiResult<HashMap<String, Value>> {
  let day = receive_day.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

  let params = json!({
    "source_id": 90139,
    "receive_day": day,
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/youth/v1/recharge/receive_vip_listen_song")
    .post()
    .header("content-type", "application/x-www-form-urlencoded")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_youth_day_vip", "请求失败")
}

#[tauri::command]
/// ## 升级 VIP
pub async fn api_youth_day_upgrade() -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let params = json!({ "kugouid": cookies.userid, "ad_type": 1 });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/youth/v1/listen_song/upgrade_vip_reward")
    .post()
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_youth_day_upgrade", "请求失败")
}
