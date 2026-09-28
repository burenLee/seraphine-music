use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{config::HttpConfig, mode::HttpMode, request::HttpRequest},
  log_err,
  utils::crypto::encrypt_rsa_unpad,
};

#[tauri::command]
/// ## 用户详情
pub async fn api_user_detail() -> ApiResult<HashMap<String, Value>> {
  let http_mode = HttpMode::get_mode();
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config(&http_mode);

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let client_time = Utc::now().timestamp();

  let pk_data = json!({ "token": token, "clienttime": client_time });
  let pk: String = encrypt_rsa_unpad(pk_data.to_string(), None)
    .map(|v| v.to_uppercase())
    .map_err(|e| {
      log_err!(e, "[api_user_detail] pk加密失败");
      "无法获取用户详情".to_string()
    })?;

  let params = json!({ "plat": 1 });
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({ "visit_time": client_time, "usertype": 1, "p": pk, "userid": userid });

  HttpRequest::new()
    .url("/v3/get_my_info")
    .post()
    .header("x-router", "usercenter.kugou.com")
    .params(params)
    .data(data)
    .builder()
    .json()
    .await
}
