use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::{ApiResult, LyricGet},
  http::{
    config::{HttpConfig, StaticConfigMode},
    request::HttpRequest,
  },
  music::lyric::{decode_lyric, Lyric, LyricFormat},
};

use crate::utils::logger::LogErrExt;

#[tauri::command]
/// ## 搜索歌词
///
/// ### 必选参数
/// * `keyword` - 搜索关键字, `artist - title` 格式
///
/// ### 可选参数
/// * `hash` - 歌曲 hash, 在线歌曲有效
/// * `album_audio_id` - 专辑音频 id
/// * `man` - 是否返回多个歌词, 默认为 `yes`
pub async fn api_lyric_search(
  keyword: &str,
  hash: Option<&str>,
  album_audio_id: Option<u8>,
  man: Option<&str>,
) -> ApiResult<HashMap<String, Value>> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let params = json!({
    "keyword": keyword,
    "hash": hash.unwrap_or_default(),
    "album_audio_id": album_audio_id.unwrap_or_default(),
    "man": man.unwrap_or("yes"),
    "appid": static_config.appid,
    "clientver": static_config.client_ver,
    "duration": 0,
    "lrctxt": 1,
  });
  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .base_url("https://lyrics.kugou.com")
    .url("/v1/search")
    .params(params)
    .clear_params(true)
    .builder()
    .json()
    .await
    .log_command("api_lyric_search", "请求失败")
}

#[tauri::command]
/// ## 获取歌词 (同时保存到本地)
///
/// ### 必选参数
/// * `id` - 歌词id
/// * `name` - 歌词名称, 用以本地保存
/// * `accesskey` - 歌词accesskey
///
/// ### 可选参数
/// * `fmt` - 歌词格式, 默认为 `krc`
/// * `decode` - 是否解码歌词, 默认为 `true`
/// * `client` - 客户端, 默认为 `android`
pub async fn api_lyric_get(
  id: &str,
  accesskey: &str,
  fmt: Option<LyricFormat>,
  decode: Option<bool>,
  client: Option<&str>,
) -> Result<Lyric, String> {
  let params = json!({
    "id": id,
    "accesskey": accesskey,
    "fmt": fmt.unwrap_or(LyricFormat::Krc),
    "client":  client.unwrap_or("android"),
    "ver": "1",
    "charset": "utf8",
  });
  let Value::Object(params) = params else { unreachable!() };

  let resp: LyricGet = HttpRequest::new()
    .base_url("https://lyrics.kugou.com")
    .url("/download")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_lyric_get", "请求失败")?;

  if resp.status != 200 {
    return Err("无法获取歌词".to_string());
  }

  // 接口返回的歌词内容的格式
  let resp_fmt = if resp.contenttype == 0 { LyricFormat::Krc } else { LyricFormat::Lrc };

  // 解码歌词
  let decoded_content = match decode.unwrap_or(true) {
    true => decode_lyric(&resp.content, &resp_fmt).log_command("api_lyric_get", "解码失败")?,
    false => String::new(),
  };

  Ok(Lyric {
    id: resp.id,
    fmt: resp_fmt,
    content: decoded_content,
  })
}
