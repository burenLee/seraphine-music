use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use tauri::{http::Method, AppHandle};

use crate::{
  api::lib::RegisterDev,
  http::{
    config::HttpConfig,
    server::{request, RequestOptions, Response, ResponseType, BASE_URL},
  },
  utils::crypto::{decrypt_aes_playlist, encrypt_aes_playlist, encrypt_rsa_pad},
};

#[tauri::command]
pub async fn api_register_dev(app_handle: AppHandle) -> Result<(), String> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  if kg_dynamic_config.cookies.dfid != "-" {
    return Ok(());
  }

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let data = json!({
    "availableRamSize": 4983533568u64, //可用内存，单位是字节
    "availableRomSize": 48114719, //内部存储可用空间，单位是字节（约48MB）
    "availableSDSize": 48114717, //外部存储可用空间，单位是字节（约48MB）
    "basebandVer":  "", //基带版本
    "batteryLevel": 100, //电池电量百分比
    "batteryStatus": 3, //电池状态
    "brand": "Redmi", //品牌
    "buildSerial":"unknown", //设备序号
    "device": "marble", //设备代号
    "imei": kg_dynamic_config.guid, //IMEI号
    "imsi": "", //sim卡号序号
    "manufacturer": "Xiaomi", //厂商
    "uuid": kg_dynamic_config.guid, //设备uuid
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

  let aes_encrypted = encrypt_aes_playlist(data.to_string()).map_err(|e| e.to_string())?;

  let p_data = json!({ "aes": aes_encrypted.key, "uid": userid, "token": token });
  let p = encrypt_rsa_pad(p_data.to_string()).map_err(|e| e.to_string())?;

  let params = json!({ "part": 1, "platid": 1, "p": p });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .base_url("https://userservice.kugou.com")
    .url("/risk/v2/r_register_dev")
    .method(Method::POST)
    .params(params)
    .data(Value::String(aes_encrypted.str))
    .response_type(ResponseType::Bytes);

  let resp = request::<Value>(opts).await.map_err(|e| e.to_string())?;
  let Response::Bytes(resp_bytes) = resp else { unreachable!() };

  // 如果是报错返回, 不需要解密处理, 所以先尝试解析
  let resp_str = String::from_utf8_lossy(&resp_bytes);
  if resp_str.starts_with('{') {
    // 暂不处理
    return Ok(());
  }

  let resp_base64 = STANDARD.encode(&resp_bytes);
  let resp_decrypted =
    decrypt_aes_playlist(&resp_base64, &aes_encrypted.key).map_err(|e| e.to_string())?;
  let resp_map = serde_json::from_str::<RegisterDev>(&resp_decrypted).map_err(|e| e.to_string())?;

  let Some(data) = &resp_map.data else { return Err(String::from("接口数据无效")) };

  let mut cookies = HttpConfig::get_kg_dynamic_config().cookies;
  cookies.dfid = data.dfid.clone();

  HttpConfig::set_kg_cookies(&app_handle, BASE_URL, cookies).map_err(|e| e.to_string())?;

  Ok(())
}
