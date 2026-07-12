use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::lib::{ApiResult, LyricGet},
  http::{
    config::HttpConfig,
    lib::KgStaticConfig,
    server::{request, RequestOptions, Response},
  },
  music::lyric::{music_lyric_save, Lyric, LyricFormat},
  utils::tools::decode_krc_lyric,
};

#[tauri::command]
/// 搜索歌词
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
  let KgStaticConfig {
    appid, client_ver, ..
  } = HttpConfig::get_kg_static_config();

  let params = json!({
    "keyword": keyword,
    "hash": hash.unwrap_or_default(),
    "album_audio_id": album_audio_id.unwrap_or_default(),
    "man": man.unwrap_or("yes"),
    "appid": appid,
    "clientver": client_ver,
    "duration": 0,
    "lrctxt": 1,
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .base_url("https://lyrics.kugou.com")
    .url("/v1/search")
    .params(params)
    .should_clear_params(true);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取歌词 (同时保存到本地)
///
/// ### 必选参数
/// * `name` - 歌词名称, 用以本地保存
/// * `id` - 歌词id
/// * `accesskey` - 歌词accesskey
///
/// ### 可选参数
/// * `fmt` - 歌词格式, 默认为 `krc`
/// * `decode` - 是否解码歌词, 默认为 `true`
/// * `client` - 客户端, 默认为 `android`
pub async fn api_lyric_get(
  name: &str,
  id: &str,
  accesskey: &str,
  fmt: Option<LyricFormat>,
  decode: Option<bool>,
  client: Option<&str>,
) -> Result<Option<Lyric>, String> {
  let params = json!({
    "id": id,
    "accesskey": accesskey,
    "fmt": fmt.unwrap_or(LyricFormat::Krc).as_ext(),
    "client":  client.unwrap_or("android"),
    "ver": "1",
    "charset": "utf8",
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .base_url("https://lyrics.kugou.com")
    .url("/download")
    .params(params);

  let resp = request::<LyricGet>(opts).await.map_err(|e| e.to_string())?;
  let Response::Json(resp_map) = resp else { unreachable!() };

  if resp_map.status != 200 {
    return Ok(None);
  }

  let lrc_decode = resp_map.fmt == "lrc" || resp_map.contenttype != 0;
  let format = if lrc_decode { LyricFormat::Lrc } else { LyricFormat::Krc };

  // 解码歌词
  let decoded_content = match decode.unwrap_or(true) {
    true => get_decode_content(&resp_map.content, lrc_decode).map_err(|e| e.to_string())?,
    false => String::new(),
  };

  // 保存歌词到本地
  music_lyric_save(name, id, &format, &resp_map.content).map_err(|e| e.to_string())?;

  Ok(Some(Lyric {
    id: resp_map.id,
    fmt: format,
    content: decoded_content,
  }))
}

/// 获取解码后的歌词内容
fn get_decode_content(content: &str, lrc_decode: bool) -> anyhow::Result<String> {
  let content_vec = STANDARD.decode(content)?;

  let decoded_content = match lrc_decode {
    true => String::from_utf8(content_vec)?,
    false => decode_krc_lyric(&content_vec)?,
  };

  Ok(decoded_content)
}
