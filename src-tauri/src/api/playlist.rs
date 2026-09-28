use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
  api::types::ApiResult,
  http::{
    config::{HttpConfig, StaticConfigMode},
    cookie::{HttpCookie, ModeCookies},
    request::HttpRequest,
  },
  utils::{
    crypto::{decrypt_aes_playlist, encrypt_aes_playlist, encrypt_rsa_pad},
    helper::sign_key_params,
    logger::LogErrExt,
  },
};

#[tauri::command]
/// ## 歌单分类
pub async fn api_playlist_tags() -> ApiResult<HashMap<String, Value>> {
  let data = json!({ "tag_type": "collection", "tag_id": 0, "source": 3 });

  HttpRequest::new()
    .url("/pubsongs/v1/get_tags_by_type")
    .post()
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_tags", "请求失败")
}

#[tauri::command]
/// ## 用户歌单
///
/// ### 可选参数
/// * `page` - 默认 1
/// * `page_size` - 默认 10
pub async fn api_playlist_user(
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let params = json!({ "plat": 1, "userid": cookies.userid, "token": cookies.token });
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({
    "userid": cookies.userid,
    "token": cookies.token,
    "total_ver": 979,
    "type": 2,
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
  });

  HttpRequest::new()
    .url("/v7/get_all_list")
    .post()
    .header("x-router", "cloudlist.service.kugou.com")
    .params(params)
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_user", "请求失败")
}

#[tauri::command]
/// ## 歌单详情
///
/// ### 必选参数
/// * `gids` - global_collection_id / list_create_gid 的集合
pub async fn api_playlist_detail(gids: Vec<&str>) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let ids: Vec<Value> = gids
    .iter()
    .map(|i| json!({ "global_collection_id": i }))
    .collect();

  let data = json!({ "data": ids, "userid": cookies.userid, "token": cookies.token });

  HttpRequest::new()
    .url("/v3/get_list_info")
    .post()
    .header("x-router", "pubsongs.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_detail", "请求失败")
}

#[tauri::command]
/// ## 收藏/新建歌单
///
/// 登录状态下可收藏已有歌单或创建新歌单。
///
/// 收藏成功后，建议使用 `/playlist/tracks/add` 接口将原歌单下的歌曲添加到新歌单。
///
/// ### 必选参数
/// * `name` - 歌单名称
/// * `userid` - 歌单创建用户id
///
/// ### 可选参数
/// * `is_pri` - 是否设为隐私: `0` 为公开, `1` 为隐私, 默认为 `0`, 该字段仅在创建歌单时有效
/// * `list_type` - 操作类型: `0` 为创建歌单，`1` 为收藏歌单, 默认为 `0`
/// * `listid` - 歌单创建列表id
/// * `gid` - 歌单创建gid
/// * `source` - 不知道什么作用
pub async fn api_playlist_add(
  name: &str,
  userid: u64,
  is_pri: Option<u64>,
  list_type: Option<u64>,
  gid: Option<u64>,
  listid: Option<u64>,
  source: Option<u64>,
) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp();

  let source = match source {
    Some(0) => 0,
    Some(source) => source,
    None => 1,
  };

  let is_pri = match list_type {
    Some(0) => is_pri.unwrap_or_default(),
    _ => 0,
  };

  let params = match list_type {
    Some(0) => {
      json!({ "last_time": client_time, "last_area": "gztx", "userid": cookies.userid, "token": cookies.token })
    }
    _ => json!({}),
  };
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({
    "userid": cookies.userid,
    "token": cookies.token,
    "name": name,
    "list_create_userid": userid,
    "is_pri": is_pri,
    "type": list_type.unwrap_or_default(),
    "list_create_listid": listid.unwrap_or_default(),
    "list_create_gid": gid.unwrap_or_default(),
    "source": source,
    "total_ver": 0,
    "from_shupinmv": 0,
  });

  HttpRequest::new()
    .url("/cloudlist.service/v5/add_list")
    .post()
    .params(params)
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_add", "请求失败")
}

#[tauri::command]
/// ## 取消收藏/删除歌单
///
/// ### 必选参数
/// * `listid` - 歌单id
pub async fn api_playlist_del(listid: u64) -> Result<HashMap<String, Value>, String> {
  let static_config = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp();

  let data = json!({ "listid": listid, "total_ver": 0, "type": 1 });
  let (aes_res, aes_key) =
    encrypt_aes_playlist(data.to_string()).log_command("api_playlist_del", "aes加密失败")?;

  let p_data = json!({ "aes": aes_key, "uid": cookies.userid, "token": cookies.token });
  let p = encrypt_rsa_pad(p_data.to_string())
    .map(|p| p.to_uppercase())
    .log_command("api_playlist_del", "p加密失败")?;

  let params = json!({
    "clienttime": client_time,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "last_area": "gztx",
    "clientver": static_config.client_ver,
    "appid": static_config.appid,
    "last_time": client_time,
    "p": p,
  });
  let Value::Object(params) = params else { unreachable!() };

  let resp_bytes = HttpRequest::new()
    .url("/v2/delete_list")
    .post()
    .header("x-router", "cloudlist.service.kugou.com")
    .params(params)
    .data(Value::String(aes_res))
    .builder()
    .bytes()
    .await
    .log_command("api_playlist_del", "请求失败")?;

  // 如果是报错返回, 不需要解密处理, 所以先尝试解析
  let resp_str = String::from_utf8_lossy(&resp_bytes);
  if resp_str.starts_with('{') {
    return serde_json::from_str::<HashMap<String, Value>>(&resp_str)
      .log_command("api_playlist_del", "序列化失败");
  }

  let resp_base64 = STANDARD.encode(&resp_bytes);
  let resp_decrypted =
    decrypt_aes_playlist(&resp_base64, &aes_key).log_command("api_playlist_del", "resp解密失败")?;
  let resp_map =
    serde_json::from_str(&resp_decrypted).log_command("api_playlist_del", "resp序列化失败")?;

  Ok(resp_map)
}

#[tauri::command]
/// ## 歌单所有歌曲(旧)
///
/// ### 必选参数
/// * `gid` - 歌单 global_collection_id / list_create_gid
///
/// ### 可选参数
/// * `page` - 页码, 默认为 1
/// * `page_size` - 每页数量, 默认为 10
pub async fn api_playlist_tracks_all(
  gid: &str,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let page = page.unwrap_or(1);
  let page_size = page_size.unwrap_or(10);

  let params = json!({
    "area_code": 1,
    "begin_idx": (page - 1) * page_size,
    "plat": 1,
    "type": 1,
    "mode": 1,
    "personal_switch": 1,
    "extend_fields": "abtags,hot_cmt,popularization",
    "pagesize": page_size,
    "global_collection_id": gid,
  });

  let Value::Object(params) = params else { unreachable!() };

  HttpRequest::new()
    .url("/pubsongs/v2/get_other_list_file_nofilt")
    .params(params)
    .builder()
    .json()
    .await
    .log_command("api_playlist_tracks_all", "请求失败")
}

#[tauri::command]
/// ## 歌单所有歌曲(新)
///
/// ### 必选参数
/// * `listid` - 歌单id
///
/// ### 可选参数
/// * `page` - 页码, 默认为 1
/// * `page_size` - 每页数量, 默认为 10
pub async fn api_playlist_tracks_all_new(
  listid: u64,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let data = json!({
    "listid": listid,
    "userid": cookies.userid,
    "area_code": 1,
    "show_relate_goods": 0,
    "pagesize": page_size.unwrap_or(10),
    "allplatform": 1,
    "show_cover": 1,
    "type": 0,
    "token": cookies.token,
    "page": page.unwrap_or(1),
  });

  HttpRequest::new()
    .url("/v4/get_list_all_file")
    .post()
    .header("x-router", "cloudlist.service.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_tracks_all_new", "请求失败")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMusicInfo {
  name: String,
  hash: String,
  album_id: Option<u64>,
  mixsongid: Option<u64>,
}

#[tauri::command]
/// ## 对歌单添加歌曲
///
/// ### 说明
/// 需要登录, 可以添加歌曲到歌单
///
/// ### 必选参数
/// * `list_id` - 用户的歌单id
/// * `music_list` - 需要添加的歌曲数据
pub async fn api_playlist_tracks_add(
  list_id: u64,
  music_list: Vec<ListMusicInfo>,
) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let client_time = Utc::now().timestamp();

  let params = json!({
    "last_time": client_time,
    "last_area": "gztx",
    "userid": cookies.userid,
    "token": cookies.token,
  });
  let Value::Object(params) = params else { unreachable!() };

  let music_list: Vec<Value> = music_list
    .into_iter()
    .map(|music| {
      json!({
        "number": 1,
        "name": music.name,
        "hash": music.hash,
        "size": 0,
        "sort": 0,
        "timelen": 0,
        "bitrate": 0,
        "album_id": music.album_id.unwrap_or_default(),
        "mixsongid": music.mixsongid.unwrap_or_default(),
      })
    })
    .collect();

  let data = json!({
    "userid": cookies.userid,
    "token": cookies.token,
    "listid": list_id,
    "list_ver": 0,
    "type": 0,
    "slow_upload": 1,
    "scene": "false;null",
    "data": music_list,
  });

  HttpRequest::new()
    .url("/cloudlist.service/v6/add_song")
    .post()
    .params(params)
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_tracks_add", "请求失败")
}

#[tauri::command]
/// ## 删除歌单歌曲
///
/// ### 必选参数
/// * `list_id` - 用户歌单id
/// * `file_ids` - 歌单中歌曲的 fileid
pub async fn api_playlist_tracks_del(
  list_id: u64,
  file_ids: Vec<u64>,
) -> ApiResult<HashMap<String, Value>> {
  let cookies = match HttpCookie::get_cookies() {
    ModeCookies::KgMobile(cookies) => cookies,
    ModeCookies::KgLite(cookies) => cookies,
  };

  let ids: Vec<Value> = file_ids.iter().map(|i| json!({ "fileid": i })).collect();
  let data = json!({
    "listid": list_id,
    "userid": cookies.userid,
    "data": ids,
    "type": 0,
    "token": cookies.token,
    "list_ver": 0,
  });

  HttpRequest::new()
    .url("/v4/delete_songs")
    .post()
    .header("x-router", "cloudlist.service.kugou.com")
    .data(data)
    .builder()
    .json()
    .await
    .log_command("api_playlist_tracks_del", "请求失败")
}
