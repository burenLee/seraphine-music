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
  utils::crypto::encrypt_rsa_unpad,
};

#[tauri::command]
pub async fn api_user_detail() -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let client_time = Utc::now().timestamp();

  let pk_data = json!({ "token": token, "clienttime": client_time });
  let pk = encrypt_rsa_unpad(pk_data.to_string(), None)
    .map(|v| v.to_uppercase())
    .map_err(|e| e.to_string())?;

  let params = json!({ "plat": 1 });
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({ "visit_time": client_time, "usertype": 1, "p": pk, "userid": userid });

  let opts = RequestOptions::new()
    .url("/v3/get_my_info")
    .method(Method::POST)
    .add_header("x-router", "usercenter.kugou.com")
    .params(params)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}
