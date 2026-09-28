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
  utils::helper::sign_key_params,
};

use crate::utils::logger::LogErrExt;

#[tauri::command]
/// ## 私人 FM(对应手机和 pc 端的猜你喜欢)
///
/// ### 可选参数
/// * `hash` - 音乐 hash, 建议
/// * `songid` - 音乐 songid, 建议
/// * `playtime` - 已播放时间, 建议
/// * `mode` - 获取模式，默认为 normal, normal：发现，small：小众，peak：30s
/// * `song_pool_id` - 手机版的 AI，0：Alpha 根据口味推荐相似歌曲, 1：Beta 根据风格推荐相似歌曲, 2：Gamma
/// * `action` - 默认为 play, garbage: 为不喜欢
/// * `is_overplay` - 是否已播放完成, 1：已播放完成, 0：未播放完成
/// * `remain_songcnt` - 剩余未播放歌曲数, 默认为 0，大于 4 不返回推荐歌曲，建议
/// * `platform` - 默认为 "ios"
pub async fn api_personal_fm(
  hash: Option<String>,
  songid: Option<String>,
  playtime: Option<String>,
  mode: Option<String>,
  action: Option<String>,
  song_pool_id: Option<u8>,
  is_overplay: Option<u8>,
  remain_songcnt: Option<u8>,
  platform: Option<String>,
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

  let client_time = Utc::now().timestamp_millis();

  let data = json!({
    "appid": static_config.appid,
    "clienttime": client_time,
    "mid": dynamic_config.mid,
    "action": action.unwrap_or("play".to_string()),
    "recommend_source_locked": 0,
    "song_pool_id": song_pool_id.unwrap_or_default(),
    "callerid": 0,
    "m_type": 1,
    "platform": platform.unwrap_or("ios".to_string()),
    "area_code": 1,
    "remain_songcnt": remain_songcnt.unwrap_or_default(),
    "clientver": static_config.client_ver,
    "is_overplay": is_overplay,
    "mode": mode.unwrap_or("normal".to_string()),
    "fakem": "ca981cfc583a4c37f28d2d49000013c16a0a",
    "key": sign_key_params(&client_time.to_string(), None, None),
    "userid": cookies.userid,
    "kguid": cookies.userid,
    "token": cookies.token,
    "vip_type": cookies.vip_type,
    "hash": hash.unwrap_or_default(),
    "songid": songid.unwrap_or_default(),
    "playtime": playtime.unwrap_or_default(),
  });

  HttpRequest::new()
    .url("/v2/personal_recommend".to_string())
    .post()
    .header("x-router", "persnfm.service.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_personal_fm", "请求失败")
}
