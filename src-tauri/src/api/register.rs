use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};

use crate::{
  api::types::RegisterDev,
  app::mode::{AppMode, Mode},
  http::{
    config::{DynamicModeConfig, HttpConfig},
    cookie::{HttpCookie, ModeCookies},
    request::HttpRequest,
  },
  utils::{
    crypto::{decrypt_aes_playlist, encrypt_aes_playlist, encrypt_rsa_pad},
    logger::LogErrExt,
  },
};

#[tauri::command]
/// ## 获取 dfid
pub async fn api_register_dev() -> Result<(), String> {
  let mut cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  // 存在就不再获取
  if cookies.dfid != "-" {
    return Ok(());
  }

  let dynamic_config = match HttpConfig::get_dynamic_config() {
    DynamicModeConfig::KgMobile(config) => config,
    DynamicModeConfig::KgLite(config) => config,
  };

  let aes_data = json!({
    "availableRamSize": 4983533568u64, //可用内存，单位是字节
    "availableRomSize": 48114719, //内部存储可用空间，单位是字节（约48MB）
    "availableSDSize": 48114717, //外部存储可用空间，单位是字节（约48MB）
    "basebandVer":  "", //基带版本
    "batteryLevel": 100, //电池电量百分比
    "batteryStatus": 3, //电池状态
    "brand": "Redmi", //品牌
    "buildSerial":"unknown", //设备序号
    "device": "marble", //设备代号
    "imei": dynamic_config.guid, //IMEI号
    "imsi": "", //sim卡号序号
    "manufacturer": "Xiaomi", //厂商
    "uuid": dynamic_config.guid, //设备uuid
    "accelerometer": false, //是否有加速度传感器
    "accelerometerValue": "", //加速度传感器值
    "gravity": false, //是否有重力传感器
    "gravityValue": "", //重力传感器的值
    "gyroscope": false, //是否有陀螺仪
    "gyroscopeValue": "", //陀螺仪的值
    "light": false, //是否有光线传感器
    "lightValue": "", //光线传感器的值
    "magnetic": false, //是否有磁力传感器
    "magneticValue": "", //磁力传感器的值
    "orientation": false, //是否有方向传感器
    "orientationValue": "", //方向传感器的值
    "pressure": false, //是否有压力传感器
    "pressureValue":  "", //压力传感器的值
    "step_counter": false, //是否有步数传感器
    "step_counterValue": "", //步数传感器的值
    "temperature": false, //是否有温度传感器
    "temperatureValue":"", //温度传感器的值
  });
  let (aes_res, aes_key) =
    encrypt_aes_playlist(aes_data.to_string()).log_command("api_register_dev", "aes加密失败")?;

  let p_data = json!({ "aes": aes_key, "uid": cookies.userid, "token": cookies.token });
  let p = encrypt_rsa_pad(p_data.to_string()).log_command("api_register_dev", "p加密失败")?;

  let params = json!({ "part": 1, "platid": 1, "p": p });
  let Value::Object(params) = params else { unreachable!() };

  let resp_bytes = HttpRequest::new()
    .base_url("https://userservice.kugou.com")
    .url("/risk/v2/r_register_dev")
    .post()
    .params(params)
    .data(Value::String(aes_res))
    .builder()
    .bytes()
    .await
    .log_command("api_register_dev", "请求失败")?;

  // 如果是报错返回, 不需要解密处理, 所以先尝试解析
  let resp_str = String::from_utf8_lossy(&resp_bytes);
  if resp_str.starts_with('{') {
    return Err(resp_str.into_owned());
  }

  let resp_base64 = STANDARD.encode(&resp_bytes);
  let resp_decrypted =
    decrypt_aes_playlist(&resp_base64, &aes_key).log_command("api_register_dev", "resp解密失败")?;
  let resp_map = serde_json::from_str::<RegisterDev>(&resp_decrypted)
    .log_command("api_register_dev", "resp序列化失败")?;

  let Some(data) = resp_map.data else {
    return Err("接口数据无效".to_string());
  };

  cookies.dfid = data.dfid.clone();

  let cookies = match AppMode::get_mode() {
    Mode::KgMobile => ModeCookies::KgMobile(cookies),
    Mode::KgLite => ModeCookies::KgLite(cookies),
  };

  HttpCookie::set_cookies(cookies).log_command("api_register_dev", "设置cookies失败")?;

  Ok(())
}
