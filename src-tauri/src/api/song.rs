use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::lib::ApiResult,
  http::{
    mode::{HttpMode, Mode},
    server::{request, RequestOptions},
  },
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Quality {
  #[serde(rename = "magic_piano")]
  MagicPiano, // 对应手机端魔法音乐 钢琴，仅部分音乐支持
  #[serde(rename = "magic_acappella")]
  MagicAcappella, // 对应手机端魔法音乐 人声 伴奏，仅部分音乐支持，该模式下返回的音频后缀为 mkv 格式，该文加存在 人声 和 伴奏 两个音轨
  #[serde(rename = "magic_subwoofer")]
  MagicSubwoofer, // 对应手机端魔法音乐 骨笛，仅部分音乐支持
  #[serde(rename = "magic_ancient")]
  MagicAncient, // 对应手机端魔法音乐 尤克里里，仅部分音乐支持
  #[serde(rename = "magic_surnay")]
  MagicSurnay, // 对应手机端魔法音乐 唢呐，仅部分音乐支持
  #[serde(rename = "magic_dj")]
  MagicDj, // 对应手机端魔法音乐 DJ，仅部分音乐支持

  #[serde(rename = "128")]
  Bitrate128, // 返回 128 码率 mp3 格式
  #[serde(rename = "320")]
  Bitrate320, // 返回 320 码率 mp3 格式
  #[serde(rename = "flac")]
  BitrateFlac, // 返回 flac 格式音频
  #[serde(rename = "high")]
  BitrateHigh, // 返回无损格式音频

  #[serde(rename = "viper_atmos")]
  ViperAtmos, // 蝰蛇全景声，仅部分音乐支持
  #[serde(rename = "viper_clear")]
  ViperClear, // 蝰蛇超清音质
  #[serde(rename = "viper_tape")]
  ViperTape, // 蝰蛇母带，仅部分音乐支持, 该音质需要转码，关于转码相关的技术还不会

  #[serde(rename = "super")]
  SuperBsd, // 返回 DSD 格式音频，支持的音频少的可伶
}

#[tauri::command]
/// 获取歌曲播放地址
///
/// ### 必选参数
/// * `hash` - 歌曲 hash
///
/// ### 可选参数
/// * `quality` - 音质，默认为 `128`
/// * `album_id` - 专辑 id
/// * `album_audio_id` - 专辑音频 id
/// * `free_part` - 是否返回试听部分（仅部分歌曲）
pub async fn api_song_url(
  hash: &str,
  quality: Option<Quality>,
  album_id: Option<u64>,
  album_audio_id: Option<u64>,
  free_part: Option<u8>,
) -> ApiResult<HashMap<String, Value>> {
  let (pid, page_id, ppage_id) = match HttpMode::get_mode() {
    Mode::KgMobile => (2, 151369488, "463467626,350369493,788954147"),
    Mode::KgLite => (411, 967177915, "356753938,823673182,967485191"),
  };

  let params = json!({
    "hash": hash.to_lowercase(),
    "quality": quality.unwrap_or(Quality::Bitrate128),
    "album_id": album_id.unwrap_or(0),
    "album_audio_id": album_audio_id.unwrap_or(0),
    "IsFreePart": free_part.unwrap_or(0),
    "pid": pid,
    "page_id": page_id,
    "ppage_id": ppage_id,
    "area_code": 1,
    "ssa_flag": "is_fromtrack",
    "version": 11430,
    "behavior": "play",
    "cmd": 26,
    "pidversion": 3001,
    "cdnBackup": 1,
    "module": "",
    "clientver": 11430,
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .url("/v5/url")
    .add_header("x-router", "trackercdn.kugou.com")
    .params(params)
    .should_encrypt(true);

  request(opts).await.map_err(|e| e.to_string())
}
