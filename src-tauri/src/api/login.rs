use chrono::{Local, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::types::{
    ApiResponse, ApiResult, LoginCellphone, LoginCellphoneData, LoginOpenplat, LoginOpenplatData,
    LoginQrCheck, LoginQrCheckData, LoginWxCheck, OpAccessToken, WxConnect, WxTicket, WxToken,
  },
  app::mode::{AppMode, Mode},
  http::{
    client::HttpClient,
    config::{DynamicModeConfig, HttpConfig, StaticConfigMode},
    cookie::{HttpCookie, ModeCookies},
    request::{EncryptType, HttpRequest},
  },
  utils::{
    crypto::{decrypt_aes, encrypt_aes, encrypt_md5, encrypt_rsa_unpad, encrypt_sha1},
    helper::sign_key_params,
    tools::gen_random_string,
  },
};

use crate::utils::logger::LogErrExt;

const LITE_T2_KEY: &str = "fd14b35e3f81af3817a20ae7adae7020";
const LITE_T2_IV: &str = "17a20ae7adae7020";
const LITE_T1_KEY: &str = "5e4ef500e9597fe004bd09a46d8add98";
const LITE_T1_IV: &str = "04bd09a46d8add98";
const APK_SIG_MD5: &str = "fe4a24d80fcf253a00676a808f62c2c6";

#[tauri::command]
/// ## 生成二维码 url
///
/// ### 必选参数
/// * `app_type` - 应用类型: `web` 为网页端, 其余为移动端, 默认为 `web`
pub async fn api_login_qr_key(app_type: Option<&str>) -> ApiResult<HashMap<String, Value>> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let app_id = if app_type == Some("web") { 1014 } else { 1001 };
  let params = json!({
    "appid": app_id,
    "type": 1,
    "plat": 4,
    "qrcode_txt": format!("https://h5.kugou.com/apps/loginQRCode/html/index.html?appid=${app_id}&"),
    "srcappid": static_config.src_appid,
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .base_url("https://login-user.kugou.com")
    .url("/v2/qrcode")
    .params(params)
    .encrypt_type(EncryptType::Web)
    .builder()
    .json()
    .await
    .log_command("api_login_qr_key", "请求失败")
}

#[tauri::command]
/// ## 创建二维码 url
///
/// ### 必选参数
/// * `key` - 二维码 key
pub fn api_login_qr_create(key: &str) -> String {
  format!("https://h5.kugou.com/apps/loginQRCode/html/index.html?qrcode={key}")
}

#[tauri::command]
/// ## 检查二维码状态
///
/// ### 必选参数
/// * `key` - 二维码 key
///
/// ### 返回结果
/// * `status` - `0`: 二维码过期，`1`: 等待扫码，`2`: 待确认，`4`: 授权登录成功（同时返回 token）
pub async fn api_login_qr_check(key: &str) -> Result<LoginQrCheck, String> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let params = json!({
    "plat": 4,
    "appid": static_config.appid,
    "srcappid": static_config.src_appid,
    "qrcode": key
  });
  let Value::Object(params) = params else { unreachable!() };

  let mut resp_map: ApiResponse<LoginQrCheckData> = HttpRequest::new()
    .base_url("https://login-user.kugou.com")
    .url("/v2/get_userinfo_qrcode")
    .params(params)
    .encrypt_type(EncryptType::Web)
    .builder()
    .json()
    .await
    .log_command("api_login_qr_check", "请求失败")?;

  let Some(data) = resp_map.data.as_mut() else {
    return Err("接口数据无效".to_string());
  };

  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(mut cookies) => {
      cookies.token = data.token.clone().unwrap_or_default();
      cookies.userid = data.userid.unwrap_or_default();

      ModeCookies::KgMobile(cookies)
    }
    ModeCookies::KgLite(mut cookies) => {
      cookies.token = data.token.clone().unwrap_or_default();
      cookies.userid = data.userid.unwrap_or_default();

      ModeCookies::KgLite(cookies)
    }
  };

  HttpCookie::set_cookies(cookies).log_command("api_login_qr_check", "设置cookies失败")?;

  data.token = None;

  Ok(resp_map)
}

#[tauri::command]
/// ## 微信登录二维码信息
pub async fn api_login_wx_create() -> Result<WxConnect, String> {
  let wx_token = wx_token().await?;
  let wx_ticket = wx_ticket(&wx_token.access_token).await?;
  let wx_connect = wx_connect(&wx_ticket.ticket).await?;

  Ok(wx_connect)
}

/// ## 微信登录的 token 信息
async fn wx_token() -> Result<WxToken, String> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let params = json!({
    "appid": static_config.wx_appid,
    "secret": static_config.wx_secret,
    "grant_type": "client_credential"
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpClient::new("https://api.weixin.qq.com/cgi-bin/token", Method::GET)
    .params(params)
    .json()
    .await
    .log_command("wx_token", "请求失败")
}

/// ## 微信登录的 ticket 信息
///
/// ### 必选参数
/// * `token` - 微信登录的 token
async fn wx_ticket(token: &str) -> Result<WxTicket, String> {
  let params = json!({ "access_token": token, "type": 2 });
  let Value::Object(params) = params else { unreachable!() };

  HttpClient::new(
    "https://api.weixin.qq.com/cgi-bin/ticket/getticket",
    Method::GET,
  )
  .params(params)
  .json()
  .await
  .log_command("wx_ticket", "请求失败")
}

/// ## 微信登录的 connect 信息
///
/// ### 必选参数
/// * `ticket` - 微信登录的 ticket
async fn wx_connect(ticket: &str) -> Result<WxConnect, String> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let timestamp = Local::now().timestamp_millis();
  let noncestr = encrypt_md5(gen_random_string(16));
  let signature = encrypt_sha1(&format!(
    "appid={}&noncestr={noncestr}&sdk_ticket={ticket}&timestamp={timestamp}",
    static_config.wx_appid
  ));

  let params = json!({
    "appid": static_config.wx_appid,
    "noncestr": noncestr,
    "timestamp": timestamp,
    "scope": "snsapi_userinfo",
    "signature": signature
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpClient::new(
    "https://open.weixin.qq.com/connect/sdk/qrconnect",
    Method::GET,
  )
  .params(params)
  .json()
  .await
  .map(|mut wx_connect: WxConnect| {
    wx_connect.qrcode.qrcodeurl = format!(
      "https://open.weixin.qq.com/connect/confirm?uuid={}",
      wx_connect.uuid
    );

    wx_connect
  })
  .log_command("wx_connect", "请求失败")
}

#[tauri::command]
/// ## 检查微信登录二维码状态
///
/// ### 必选参数
/// * `uuid` - 微信登录二维码的 uuid
///
/// ### 返回结果
/// * `status` - `402`: 已过期, `403`: 拒绝登录, `404`: 已经扫描, `405`: 登录成功(返回 wx_code), `408`: 等待扫描
pub async fn api_login_wx_check(uuid: &str) -> Result<LoginWxCheck, String> {
  HttpClient::new(
    format!("https://long.open.weixin.qq.com/connect/l/qrconnect?f=json&uuid={uuid}"),
    Method::GET,
  )
  .json()
  .await
  .log_command("api_login_wx_check", "请求失败")
}

#[tauri::command]
/// ## 开放平台登录, 目前仅支持微信
///
/// ### 必选参数
/// * `code` - 第三方的 code
pub async fn api_login_openplat(code: &str) -> Result<LoginOpenplat, String> {
  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };

  let client_time = Utc::now().timestamp_millis();

  let op_access_token = op_access_token(code).await?;
  let aes_json = json!({ "access_token": op_access_token.access_token });
  let (aes_res, aes_key) = encrypt_aes(aes_json.to_string(), None, None)
    .log_command("api_login_openplat", "aes加密失败")?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": aes_key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .log_command("api_login_openplat", "pk加密失败")?;

  let (t1, t2) = match AppMode::get_mode() {
    Mode::KgMobile => ("0".to_string(), "0".to_string()),
    Mode::KgLite => {
      let (t1_res, _) = encrypt_aes(
        format!("|{client_time}"),
        Some(LITE_T1_KEY),
        Some(LITE_T1_IV),
      )
      .log_command("api_login_openplat", "t1加密失败")?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{client_time}",
        dynamic_config.guid, dynamic_config.mac, dynamic_config.dev,
      );
      let (t2_res, _) = encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV))
        .log_command("api_login_openplat", "t2加密失败")?;

      (t1_res, t2_res)
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
    "params": aes_res,
    "pk": pk,
  });

  let mut resp: ApiResponse<LoginOpenplatData> = HttpRequest::new()
    .url("/v6/login_by_openplat")
    .post()
    .header("x-router", "login.user.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_login_openplat", "请求失败")?;

  let Some(data) = resp.data.as_mut() else {
    return Err("接口数据无效".to_string());
  };

  let decrypted_token = decrypt_aes(&data.secu_params, &aes_key, None)
    .log_command("api_login_openplat", "token解密失败")?;
  let token = match serde_json::from_str::<HashMap<String, String>>(&decrypted_token) {
    Ok(token_map) => token_map
      .get("token")
      .map(|s| s.to_string())
      .ok_or("未获取到token")?,
    Err(_) => decrypted_token,
  };

  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(mut cookies) => {
      cookies.t1 = data.t1.clone();
      cookies.userid = data.userid;
      cookies.vip_type = data.vip_type;
      cookies.vip_token = data.vip_token.clone();
      cookies.token = token;

      ModeCookies::KgMobile(cookies)
    }
    ModeCookies::KgLite(mut cookies) => {
      cookies.t1 = data.t1.clone();
      cookies.userid = data.userid;
      cookies.vip_type = data.vip_type;
      cookies.vip_token = data.vip_token.clone();
      cookies.token = token;

      ModeCookies::KgLite(cookies)
    }
  };

  HttpCookie::set_cookies(cookies).log_command("api_login_openplat", "设置cookies失败")?;

  data.t1 = String::new();
  data.vip_type = 0;
  data.vip_token = String::new();
  data.secu_params = String::new();
  data.token = None;

  Ok(resp)
}

/// ## 开放平台的 access_token 信息
///
/// ### 必选参数
/// * `code` - 第三方登录的 code
async fn op_access_token(code: &str) -> Result<OpAccessToken, String> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let params = json!({
    "secret": static_config.wx_secret,
    "appid": static_config.wx_appid,
    "code": code,
    "grant_type": "authorization_code",
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpClient::new(
    "https://api.weixin.qq.com/sns/oauth2/access_token",
    Method::POST,
  )
  .params(params)
  .json()
  .await
  .log_command("op_access_token", "请求失败")
}

#[tauri::command]
/// ## 手机验证码
///
/// 对应 `captcha_sent`
///
/// ### 必选参数
/// * `mobile` - 手机号
pub async fn api_login_captcha(mobile: &str) -> ApiResult<HashMap<String, Value>> {
  let data = json!({ "businessid": 5, "mobile": mobile, "plat": 3 });

  HttpRequest::new()
    .base_url("http://login.user.kugou.com")
    .url("/v7/send_mobile_code")
    .post()
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_login_captcha", "请求失败")
}

#[tauri::command]
/// ## 手机号登录
///
/// ### 必选参数
/// * `mobile` - 手机号
/// * `code` - 验证码
///
/// ### 可选参数
/// * `userid` - 用户 id, 默认为 0
pub async fn api_login_cellphone(
  mobile: &str,
  code: &str,
  userid: Option<&str>,
) -> Result<LoginCellphone, String> {
  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };

  let client_time = Local::now().timestamp_millis();

  let aes_json = json!({ "mobile": mobile, "code": code });
  let (aes_res, aes_key) = encrypt_aes(aes_json.to_string(), None, None)
    .log_command("api_login_cellphone", "aes加密失败")?;

  let mobile = format!("{}*****{}", &mobile[0..2], &mobile[10..11]);
  let dfid = gen_random_string(24);

  let (t1, t2) = match AppMode::get_mode() {
    Mode::KgMobile => ("0".to_string(), "0".to_string()),
    Mode::KgLite => {
      let (t1_res, _) = encrypt_aes(
        format!("|{client_time}"),
        Some(LITE_T1_KEY),
        Some(LITE_T1_IV),
      )
      .log_command("api_login_cellphone", "t1加密失败")?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{}",
        dynamic_config.guid, dynamic_config.mac, dynamic_config.dev, client_time
      );
      let (t2_res, _) = encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV))
        .log_command("api_login_cellphone", "t2加密失败")?;

      (t1_res, t2_res)
    }
  };

  let pk_json = json!({ "clienttime_ms": client_time, "key": aes_key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .log_command("api_login_cellphone", "pk加密失败")?;

  let data = json!({
    "plat": 1,
    "support_multi": 1,
    "t1": t1,
    "t2": t2,
    "clienttime_ms": client_time,
    "mobile": mobile,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "pk": pk,
    "params": aes_res
  });
  let Value::Object(mut data) = data else { unreachable!() };

  if let Some(user_id) = userid {
    data.insert("userid".to_string(), json!(user_id));
  }

  match AppMode::get_mode() {
    Mode::KgMobile => {
      data.insert("t3".to_string(), json!("MCwwLDAsMCwwLDAsMCwwLDA"));
    }
    Mode::KgLite => {
      data.insert("dfid".to_string(), json!(dfid));
      data.insert("dev".to_string(), json!(dynamic_config.dev));
      data.insert("gitversion".to_string(), json!("5f0b7c4"));
    }
  }

  let mut resp: ApiResponse<LoginCellphoneData> = HttpRequest::new()
    .base_url("https://loginserviceretry.kugou.com")
    .url("/v7/login_by_verifycode")
    .post()
    .header("support-calm", "1")
    .header("user-agent", "Android16-1070-11440-130-0-LOGIN-wifi")
    .data(Value::Object(data))
    .builder()
    .json()
    .await
    .log_command("api_login_cellphone", "请求失败")?;

  let Some(data) = resp.data.as_mut() else {
    return Err("接口数据无效".to_string());
  };

  let decrypted_token = decrypt_aes(&data.secu_params, &aes_key, None)
    .log_command("api_login_cellphone", "token解密失败")?;
  let token = match serde_json::from_str::<HashMap<String, String>>(&decrypted_token) {
    Ok(token_map) => token_map
      .get("token")
      .map(|s| s.to_string())
      .ok_or("未获取到token")?,
    Err(_) => decrypted_token,
  };

  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(mut cookies) => {
      cookies.t1 = data.t1.clone();
      cookies.userid = data.userid;
      cookies.vip_type = data.vip_type;
      cookies.vip_token = data.vip_token.clone();
      cookies.token = token;

      ModeCookies::KgMobile(cookies)
    }
    ModeCookies::KgLite(mut cookies) => {
      cookies.t1 = data.t1.clone();
      cookies.userid = data.userid;
      cookies.vip_type = data.vip_type;
      cookies.vip_token = data.vip_token.clone();
      cookies.token = token;

      ModeCookies::KgLite(cookies)
    }
  };

  HttpCookie::set_cookies(cookies).log_command("api_login_cellphone", "设置cookies失败")?;

  data.t1 = String::new();
  data.vip_type = 0;
  data.vip_token = String::new();
  data.secu_params = String::new();
  data.token = None;

  Ok(resp)
}

#[tauri::command]
/// ## 刷新登录
pub async fn api_login_token() -> ApiResult<HashMap<String, Value>> {
  let http_mode = AppMode::get_mode();
  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp_millis();

  let (key, iv) = match http_mode {
    Mode::KgMobile => ("90b8382a1bb4ccdcf063102053fd75b8", "f063102053fd75b8"),
    Mode::KgLite => ("c24f74ca2820225badc01946dba4fdf7", "adc01946dba4fdf7"),
  };

  let aes_json = json!({ "clienttime": client_time / 1000, "token": cookies.token });
  let (aes_res, _) = encrypt_aes(aes_json.to_string(), Some(key), Some(iv))
    .log_command("api_login_token", "aes加密失败")?;

  let (params_aes_res, params_aes_key) = encrypt_aes(format!("{{}}"), None, None)
    .log_command("api_login_token", "params_aes加密失败")?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": params_aes_key });
  let pk =
    encrypt_rsa_unpad(pk_json.to_string(), None).log_command("api_login_token", "pk加密失败")?;

  let (t1, t2) = match http_mode {
    Mode::KgMobile => ("0".to_string(), "0".to_string()),
    Mode::KgLite => {
      let t1_str = format!("{}|{client_time}", cookies.t1);
      let (t1_res, _) = encrypt_aes(t1_str, Some(LITE_T1_KEY), Some(LITE_T1_IV))
        .log_command("api_login_token", "t1加密失败")?;

      let t2_str = format!(
        "{}|0f607264fc6318a92b9e13c65db7cd3c|{}|{}|{client_time}",
        dynamic_config.guid, dynamic_config.mac, dynamic_config.dev
      );
      let (t2_res, _) = encrypt_aes(t2_str, Some(LITE_T2_KEY), Some(LITE_T2_IV))
        .log_command("api_login_token", "t2加密失败")?;

      (t1_res, t2_res)
    }
  };

  let data = json!({
    "dfid": cookies.dfid,
    "p3": aes_res,
    "plat": 1,
    "t1": t1,
    "t2": t2,
    "t3": "MCwwLDAsMCwwLDAsMCwwLDA",
    "pk": pk,
    "params": params_aes_res,
    "userid": cookies.userid,
    "clienttime_ms": client_time,
  });

  HttpRequest::new()
    .base_url("http://login.user.kugou.com")
    .url("/v5/login_by_token")
    .post()
    .header("x-router", "login.user.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_login_token", "请求失败")
}

#[tauri::command]
/// ## 设备列表
pub async fn api_login_device() -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp_millis();

  let aes_json = json!({ "token": cookies.token });
  let (aes_res, aes_key) =
    encrypt_aes(aes_json.to_string(), None, None).log_command("api_login_device", "aes加密失败")?;

  let pk_json = json!({ "clienttime_ms": client_time, "key": aes_key });
  let pk = encrypt_rsa_unpad(pk_json.to_string(), None)
    .map(|p| p.to_uppercase())
    .log_command("api_login_device", "pk加密失败")?;

  let data = json!({
    "plat": 1,
    "userid": cookies.userid,
    "clienttime_ms": client_time,
    "pk": pk,
    "params": aes_res,
  });

  HttpRequest::new()
    .base_url("https://userinfoservice.kugou.com")
    .url("/v2/get_dev")
    .post()
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_login_device", "请求失败")
}

#[tauri::command]
/// ## 设备登出
pub async fn api_login_device_kick() -> ApiResult<HashMap<String, Value>> {
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
  let dfid = "-";

  let aes_json = json!({ "token": cookies.token });
  let (aes_res, _) = encrypt_aes(aes_json.to_string(), None, None)
    .log_command("api_login_device_kick", "aes加密失败")?;

  let data = json!({
    "appid": static_config.appid,
    "clientver": static_config.client_ver,
    "clienttime": client_time,
    "mid": dynamic_config.mid,
    "uuid": dynamic_config.guid,
    "dfid": dfid,
    "plat": 1,
    "userid": cookies.userid,
    "token": aes_res,
    "t_mid": dynamic_config.guid,
    "t": client_time,
    "t_appid": 3116,
    "t_clientver": 10597,
    "srcappid": static_config.src_appid,
    "signature": sign_key_params(&client_time.to_string(), None, None),
  });

  HttpRequest::new()
    .url("/loginservice/v1/dev_logout")
    .header("host", "gateway.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_login_device_kick", "请求失败")
}

#[tauri::command]
/// ## 登出账号
pub fn api_login_out() -> Result<(), String> {
  HttpCookie::clear_cookies().log_command("api_login_out", "清除cookie失败")
}

#[tauri::command]
/// ## 账号是否在线
///
/// 判断依据为cookies是否有token(无法判断是否过期)
///
/// 用来统一前后端登录状态, 因为后端可能出现:
///
/// 登录成功设置了cookies, 但是没有保存到store中, 导致下次启动时登录状态丢失, 与前端的登录状态不同步
pub fn api_login_online() -> bool {
  match AppMode::get_mode() {
    Mode::KgMobile => {
      if let ModeCookies::KgMobile(kg_cookies) = HttpCookie::get_cookies() {
        if !kg_cookies.token.is_empty() {
          return true;
        }
      }
    }
    Mode::KgLite => {
      if let ModeCookies::KgLite(kg_cookies) = HttpCookie::get_cookies() {
        if !kg_cookies.token.is_empty() {
          return true;
        }
      }
    }
  }

  false
}
