use chrono::Local;
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri::http::Method;

use crate::{
  api::lib::ApiResult,
  http::{
    config::HttpConfig,
    server::{request, RequestOptions},
  },
};

#[tauri::command]
/// 获取已领取 VIP 状态
pub async fn api_youth_union_vip() -> ApiResult<HashMap<String, Value>> {
  let params = json!({
    "busi_type": "concept",
    "opt_product_types": "dvip,qvip",
    "product_type": "svip"
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .base_url("https://kugouvip.kugou.com")
    .url("/v1/get_union_vip")
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 领取 VIP
///
/// ### 必选参数：
/// * `receive_day` - 领取 VIP 日期，格式为：2026-01-30
pub async fn api_youth_day_vip(receive_day: Option<String>) -> ApiResult<HashMap<String, Value>> {
  let day = receive_day.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

  let params = json!({
    "source_id": 90139,
    "receive_day": day,
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .url("/youth/v1/recharge/receive_vip_listen_song")
    .method(Method::POST)
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 升级 VIP
pub async fn api_youth_day_upgrade() -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let params = json!({
    "kugouid": kg_dynamic_config.cookies.userid,
    "ad_type": 1,
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .url("/youth/v1/listen_song/upgrade_vip_reward")
    .method(Method::POST)
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}
