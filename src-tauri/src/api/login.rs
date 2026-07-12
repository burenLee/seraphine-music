use chrono::{Local, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::lib::{
    ApiResult, LoginCellphone, LoginOpenplat, LoginQrCheck, LoginWxCheck, OpAccessToken, WxConnect,
    WxTicket, WxToken,
  },
  http::{
    client::{HttpRequest, HttpRequestOptions},
    config::HttpConfig,
    mode::{HttpMode, Mode},
    server::{request, EncryptType, RequestOptions, Response, BASE_URL},
  },
  utils::{
    crypto::{decrypt_aes, encrypt_aes, encrypt_md5, encrypt_rsa_unpad, encrypt_sha1},
    helper::sign_key_params,
    tools::random_string,
  },
};

const LITE_T2_KEY: &str = "fd14b35e3f81af3817a20ae7adae7020";
const LITE_T2_IV: &str = "17a20ae7adae7020";
const LITE_T1_KEY: &str = "5e4ef500e9597fe004bd09a46d8add98";
const LITE_T1_IV: &str = "04bd09a46d8add98";

#[tauri::command]
/// 生成二维码url
///
/// ## 必选参数
///
/// * `app_type` - 应用类型: `web`, 默认为 `web`
pub async fn api_login_qr_key(app_type: Option<&str>) -> ApiResult<HashMap<String, Value>> {
  let kg_static_config = HttpConfig::get_kg_static_config();

  let app_id = match app_type == Some("web") {
    true => 1014,
    false => 1001,
  };

  let params = json!({
    "appid": app_id,
    "type": 1,
    "plat": 4,
    "qrcode_txt": format!("https://h5.kugou.com/apps/loginQRCode/html/index.html?appid=${app_id}&"),
    "srcappid": kg_static_config.src_appid,
  });

  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .base_url("https://login-user.kugou.com")
    .url("/v2/qrcode")
    .params(params)
    .encrypt_type(EncryptType::Web);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 创建二维码url
pub fn api_login_qr_create(key: &str) -> String {
  format!("https://h5.kugou.com/apps/loginQRCode/html/index.html?qrcode={key}")
}

#[tauri::command]
/// 检查二维码状态
///
/// ## 必选参数
///
/// * `key` - 二维码 key
///
/// ## 返回结果
///
/// * `status` - `0`: 二维码过期，`1`: 等待扫码，`2`: 待确认，`4`: 授权登录成功（同时返回 token）
pub async fn api_login_qr_check(app_handle: AppHandle, key: &str) -> Result<LoginQrCheck, String> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();
  let kg_static_config = HttpConfig::get_kg_static_config();

  let params = json!({
    "plat": 4,
    "appid": kg_static_config.appid,
    "srcappid": kg_static_config.src_appid,
    "qrcode": key
  });
  let Value::Object(params) = params else { unreachable!() };

  let base_url = String::from("https://login-user.kugou.com");

  let opts = RequestOptions::new()
    .base_url(&base_url)
    .url(String::from("/v2/get_userinfo_qrcode"))
    .params(params)
    .encrypt_type(EncryptType::Web);

  let resp = request::<LoginQrCheck>(opts)
    .await
    .map_err(|e| e.to_string())?;

  let Response::Json(mut resp_map) = resp else { unreachable!() };

  let Some(data) = resp_map.data.as_mut() else {
    return Err(String::from("接口数据无效"));
  };

  let mut cookies = kg_dynamic_config.cookies;
  cookies.token = data.token.clone().unwrap_or_default();
  cookies.userid = data.userid.unwrap_or_default();
  HttpConfig::set_kg_cookies(&app_handle, &base_url, cookies).map_err(|e| e.to_string())?;

  data.token = None;

  Ok(resp_map)
}

#[tauri::command]
/// 获取微信登录二维码信息
pub async fn api_login_wx_create() -> Result<WxConnect, String> {
  let kg_static_config = HttpConfig::get_kg_static_config();

  let wx_token = wx_token(kg_static_config.wx_appid, kg_static_config.wx_secret).await?;
  let wx_ticket = wx_ticket(&wx_token.access_token).await?;
  let wx_connect = wx_connect(&wx_ticket.ticket, kg_static_config.wx_appid).await?;

  Ok(wx_connect)
}

/// 获取微信登录的 token 信息
async fn wx_token(wx_appid: &str, wx_secret: &str) -> Result<WxToken, String> {
  let params = json!({
    "appid": wx_appid,
    "secret": wx_secret,
    "grant_type": "client_credential"
  });
  let Value::Object(params) = params else { unreachable!() };

  let http_opts = HttpRequestOptions::new()
    .url("https://api.weixin.qq.com/cgi-bin/token")
    .params(params);

  let resp = HttpRequest::request(http_opts)
    .await
    .map_err(|e| e.to_string())?;

  resp.json().await.map_err(|e| e.to_string())
}

/// 获取微信登录的 ticket 信息
async fn wx_ticket(token: &str) -> Result<WxTicket, String> {
  let params = json!({ "access_token": token, "type": 2 });
  let Value::Object(params) = params else { unreachable!() };

  let http_opts = HttpRequestOptions::new()
    .url("https://api.weixin.qq.com/cgi-bin/ticket/getticket")
    .params(params);

  let resp = HttpRequest::request(http_opts)
    .await
    .map_err(|e| e.to_string())?;

  resp.json().await.map_err(|e| e.to_string())
}

/// 获取微信登录的 connect 信息
async fn wx_connect(ticket: &str, wx_appid: &str) -> Result<WxConnect, String> {
  let timestamp = Local::now().timestamp_millis();
  let noncestr = encrypt_md5(random_string(16));
  let signature = encrypt_sha1(&format!(
    "appid={wx_appid}&noncestr={noncestr}&sdk_ticket={ticket}&timestamp={timestamp}"
  ));

  let params = json!({
    "appid": wx_appid,
    "noncestr": noncestr,
    "timestamp": timestamp,
    "scope": "snsapi_userinfo",
    "signature": signature
  });
  let Value::Object(params) = params else { unreachable!() };

  let http_opts = HttpRequestOptions::new()
    .url("https://open.weixin.qq.com/connect/sdk/qrconnect")
    .params(params);

  let resp = HttpRequest::request(http_opts)
    .await
    .map_err(|e| e.to_string())?;

  let mut wx_connect: WxConnect = resp.json().await.map_err(|e| e.to_string())?;

  wx_connect.qrcode.qrcodeurl = format!(
    "https://open.weixin.qq.com/connect/confirm?uuid={}",
    wx_connect.uuid
  );

  Ok(wx_connect)
}

#[tauri::command]
/// 检查微信登录二维码状态
///
/// ## 必选参数
///
/// * `uuid` - 微信登录二维码的 uuid
///
/// ## 返回结果
///
/// * `status` - `402`: 已过期, `403`: 拒绝登录, `404`: 已经扫描, `405`: 登录成功(返回 wx_code), `408`: 等待扫描
pub async fn api_login_wx_check(uuid: &str) -> Result<LoginWxCheck, String> {
  HttpRequest::get_json(format!(
    "https://long.open.weixin.qq.com/connect/l/qrconnect?f=json&uuid={uuid}"
  ))
  .await
  .map_err(|e| e.to_string())
}

#[tauri::command]
/// 开放平台登录, 目前仅支持微信
///
/// ## 必选参数
///
/// * `code` - 第三方的 code
pub async fn api_login_openplat(
  app_handle: AppHandle,
  code: &str,
) -> Result<LoginOpenplat, String> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();
  let kg_static_config = HttpConfig::get_kg_static_config();

  let op_access_token =
    op_access_token(code, kg_static_config.wx_appid, kg_static_config.wx_secret).await?;

  let client_time = Utc::now().timestamp_millis();

  let encrypt_json = json!({ "access_token": op_access_token.access_token });
  let encrypted = encrypt_aes(encrypt_json.to_string(), None, None).map_err(|e| e.to_string())?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": encrypted.key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .map_err(|e| e.to_string())?;

  let (t1, t2) = match HttpMode::get_mode() {
    Mode::KgMobile => (String::from("0"), String::from("0")),
    Mode::KgLite => {
      let t1 = encrypt_aes(
        format!("|{client_time}"),
        Some(LITE_T1_KEY),
        Some(LITE_T1_IV),
      )
      .map_err(|e| e.to_string())?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{}",
        kg_dynamic_config.guid, kg_dynamic_config.mac, kg_dynamic_config.dev, client_time
      );
      let t2 =
        encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV)).map_err(|e| e.to_string())?;

      (t1.str, t2.str)
    }
  };

  let data = json!({
    "force_login": 1,
    "partnerid": 36,
    "clienttime_ms": client_time,
    "t1": t1,
    "t2": t2,
    "t3": "MCwwLDAsMCwwLDAsMCwwLDA=",
    "openid": op_access_token.openid,
    "params": encrypted.str,
    "pk": pk,
  });

  let opts = RequestOptions::new()
    .url("/v6/login_by_openplat")
    .method(Method::POST)
    .add_header("x-router", "login.user.kugou.com")
    .data(data);

  let resp: Response<LoginOpenplat> = request(opts).await.map_err(|e| e.to_string())?;

  let Response::Json(mut resp_map) = resp else {
    return Err(String::from("接口数据无效"));
  };

  let Some(data) = resp_map.data.as_mut() else {
    return Err(String::from("接口数据无效"));
  };

  let mut cookies = kg_dynamic_config.cookies;
  cookies.t1 = data.t1.clone();
  cookies.userid = data.userid;
  cookies.vip_type = data.vip_type;
  cookies.vip_token = data.vip_token.clone();

  let get_token =
    decrypt_aes(&data.secu_params, &encrypted.key, None).map_err(|e| e.to_string())?;

  if let Ok(token_map) = serde_json::from_str::<HashMap<String, String>>(&get_token) {
    cookies.token = token_map
      .get("token")
      .map(|s| s.to_string())
      .unwrap_or_default();
  } else {
    cookies.token = get_token;
  }

  HttpConfig::set_kg_cookies(&app_handle, BASE_URL, cookies).map_err(|e| e.to_string())?;

  data.t1 = String::new();
  data.vip_type = 0;
  data.vip_token = String::new();
  data.secu_params = String::new();
  data.token = None;

  Ok(resp_map)
}

/// 获取开放平台的 access_token 信息
async fn op_access_token(
  code: &str,
  wx_appid: &str,
  wx_secret: &str,
) -> Result<OpAccessToken, String> {
  let params = json!({
    "secret": wx_secret,
    "appid": wx_appid,
    "code": code,
    "grant_type": "authorization_code",
  });

  let Value::Object(params) = params else { unreachable!() };

  let http_opts = HttpRequestOptions::new()
    .url("https://api.weixin.qq.com/sns/oauth2/access_token")
    .method(Method::POST)
    .params(params);

  let resp = HttpRequest::request(http_opts)
    .await
    .map_err(|e| e.to_string())?;

  resp.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取验证码
///
/// ## 必选参数
///
/// * `mobile` - 手机号
pub async fn api_login_captcha(mobile: &str) -> ApiResult<HashMap<String, Value>> {
  let data = json!({
    "businessid": 5,
    "mobile": mobile,
    "plat": 3,
  });

  let opts = RequestOptions::new()
    .base_url("http://login.user.kugou.com")
    .url("/v7/send_mobile_code")
    .method(Method::POST)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 手机号登录
///
/// ## 必选参数
///
/// * `mobile` - 手机号
/// * `code` - 验证码
///
/// ## 可选参数
///
/// * `userid` - 用户 id, 默认为 0
pub async fn api_login_cellphone(
  app_handle: AppHandle,
  mobile: &str,
  code: &str,
  userid: Option<&str>,
) -> Result<LoginCellphone, String> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let client_time = Local::now().timestamp_millis();

  let encrypt_json = json!({ "mobile": mobile, "code": code });
  let encrypted = encrypt_aes(encrypt_json.to_string(), None, None).map_err(|e| e.to_string())?;

  let mobile = format!("{}*****{}", &mobile[0..2], &mobile[10..11]);
  let dfid = random_string(24);

  let (t1, t2) = match HttpMode::get_mode() {
    Mode::KgMobile => (String::from("0"), String::from("0")),
    Mode::KgLite => {
      let t1 = encrypt_aes(
        format!("|{client_time}"),
        Some(LITE_T1_KEY),
        Some(LITE_T1_IV),
      )
      .map_err(|e| e.to_string())?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{}",
        kg_dynamic_config.guid, kg_dynamic_config.mac, kg_dynamic_config.dev, client_time
      );
      let t2 =
        encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV)).map_err(|e| e.to_string())?;

      (t1.str, t2.str)
    }
  };

  let pk_json = json!({ "clienttime_ms": client_time, "key": encrypted.key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .map_err(|e| e.to_string())?;

  let data = json!({
    "plat": 1,
    "support_multi": 1,
    "t1": t1,
    "t2": t2,
    "clienttime_ms": client_time,
    "mobile": mobile,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "pk": pk,
    "params": encrypted.str
  });
  let Value::Object(mut data) = data else { unreachable!() };

  if let Some(user_id) = userid {
    data.insert(String::from("userid"), json!(user_id));
  }

  match HttpMode::get_mode() {
    Mode::KgMobile => {
      data.insert(String::from("t3"), json!("MCwwLDAsMCwwLDAsMCwwLDA"));
    }
    Mode::KgLite => {
      data.insert(String::from("dfid"), json!(dfid));
      data.insert(String::from("dev"), json!(kg_dynamic_config.dev));
      data.insert(String::from("gitversion"), json!("5f0b7c4"));
    }
  }

  let opts = RequestOptions::new()
    .base_url("https://loginserviceretry.kugou.com")
    .url("/v7/login_by_verifycode")
    .method(Method::POST)
    .add_header("support-calm", "1")
    .add_header("user-agent", "Android16-1070-11440-130-0-LOGIN-wifi")
    .data(Value::Object(data));

  let resp: Response<LoginCellphone> = request(opts).await.map_err(|e| e.to_string())?;
  let Response::Json(mut resp) = resp else { unreachable!() };

  let Some(data) = resp.data.as_mut() else {
    return Err(String::from("接口数据无效"));
  };

  let mut cookies = kg_dynamic_config.cookies;
  cookies.t1 = data.t1.clone();
  cookies.userid = data.userid;
  cookies.vip_type = data.vip_type;
  cookies.vip_token = data.vip_token.clone();

  let get_token =
    decrypt_aes(&data.secu_params, &encrypted.key, None).map_err(|e| e.to_string())?;

  if let Ok(token_map) = serde_json::from_str::<HashMap<String, String>>(&get_token) {
    cookies.token = token_map
      .get("token")
      .map(|s| s.to_string())
      .unwrap_or_default();
  } else {
    cookies.token = get_token;
  }

  HttpConfig::set_kg_cookies(&app_handle, BASE_URL, cookies).map_err(|e| e.to_string())?;

  data.t1 = String::new();
  data.vip_type = 0;
  data.vip_token = String::new();
  data.secu_params = String::new();
  data.token = None;

  Ok(resp)
}

#[tauri::command]
/// 刷新登录
pub async fn api_login_token() -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let client_time = Utc::now().timestamp_millis();
  let dfid = kg_dynamic_config.cookies.dfid;
  let user_id = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let (key, iv) = match HttpMode::get_mode() {
    Mode::KgMobile => ("90b8382a1bb4ccdcf063102053fd75b8", "f063102053fd75b8"),
    Mode::KgLite => ("c24f74ca2820225badc01946dba4fdf7", "adc01946dba4fdf7"),
  };

  let encrypt_json = json!({ "clienttime": client_time / 1000, "token": token });
  let encrypted =
    encrypt_aes(encrypt_json.to_string(), Some(key), Some(iv)).map_err(|e| e.to_string())?;

  let encrypted_params = encrypt_aes(format!("{{}}"), None, None).map_err(|e| e.to_string())?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": encrypted_params.key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None).map_err(|e| e.to_string())?;

  let (t1, t2) = match HttpMode::get_mode() {
    Mode::KgMobile => (String::from("0"), String::from("0")),
    Mode::KgLite => {
      let t1_str = if kg_dynamic_config.cookies.t1.is_empty() {
        format!("|{client_time}")
      } else {
        format!("{}|{client_time}", kg_dynamic_config.cookies.t1)
      };
      let t1 =
        encrypt_aes(t1_str, Some(LITE_T1_KEY), Some(LITE_T1_IV)).map_err(|e| e.to_string())?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{}",
        kg_dynamic_config.guid, kg_dynamic_config.mac, kg_dynamic_config.dev, client_time
      );
      let t2 =
        encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV)).map_err(|e| e.to_string())?;

      (t1.str, t2.str)
    }
  };

  let data = json!({
    "dfid":dfid,
    "p3": encrypted.str,
    "plat": 1,
    "t1": t1,
    "t2": t2,
    "t3": "MCwwLDAsMCwwLDAsMCwwLDA",
    "pk": pk,
    "params": encrypted_params.str,
    "userid": user_id,
    "clienttime_ms": client_time,
  });

  let opts = RequestOptions::new()
    .base_url("http://login.user.kugou.com")
    .url("/v5/login_by_token")
    .method(Method::POST)
    .add_header("x-router", "login.user.kugou.com")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn api_login_device() -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let user_id = kg_dynamic_config.cookies.userid;
  let client_time = Utc::now().timestamp_millis();

  let encrypt_json = json!({ "token": kg_dynamic_config.cookies.token });
  let encrypted = encrypt_aes(encrypt_json.to_string(), None, None).map_err(|e| e.to_string())?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": encrypted.key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .map_err(|e| e.to_string())?;

  let data = json!({
    "plat": 1,
    "userid": user_id,
    "clienttime_ms": client_time,
    "pk": pk,
    "params": encrypted.str,
  });

  let opts = RequestOptions::new()
    .base_url("https://userinfoservice.kugou.com")
    .url("/v2/get_dev")
    .method(Method::POST)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn api_login_device_kick() -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();
  let kg_static_config = HttpConfig::get_kg_static_config();

  let client_time = Utc::now().timestamp_millis();

  let dfid = "-";
  let user_id = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let encrypt_json = json!({ "token": token });
  let encrypted = encrypt_aes(encrypt_json.to_string(), None, None).map_err(|e| e.to_string())?;

  let data = json!({
    "appid": kg_static_config.appid,
    "clientver": kg_static_config.client_ver,
    "clienttime": client_time,
    "mid": kg_dynamic_config.mid,
    "uuid": kg_dynamic_config.guid,
    "dfid": dfid,
    "plat": 1,
    "userid": user_id,
    "token": encrypted.str,
    "t_mid": kg_dynamic_config.guid,
    "t": client_time,
    "t_appid": 3116,
    "t_clientver": 10597,
    "srcappid": kg_static_config.src_appid,
    "signature": sign_key_params(&client_time.to_string(), None, None),
  });

  let opts = RequestOptions::new()
    .url("/loginservice/v1/dev_logout")
    .add_header("host", "gateway.kugou.com")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 登出账号
pub fn api_login_out(app_handle: AppHandle) -> Result<(), String> {
  HttpConfig::clear_kg_cookies(&app_handle, BASE_URL).map_err(|e| e.to_string())
}
