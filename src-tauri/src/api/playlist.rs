use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_plugin_http::reqwest::Method;

use crate::{
  api::lib::ApiResult,
  http::{
    config::HttpConfig,
    server::{request, RequestOptions, Response, ResponseType},
  },
  utils::{
    crypto::{decrypt_aes_playlist, encrypt_aes_playlist, encrypt_rsa_pad},
    helper::sign_key_params,
  },
};

#[tauri::command]
/// 歌单分类
pub async fn api_playlist_tags() -> ApiResult<HashMap<String, Value>> {
  let data = json!({ "tag_type": "collection", "tag_id": 0, "source": 3 });

  let opts = RequestOptions::new()
    .url("/pubsongs/v1/get_tags_by_type")
    .method(Method::POST)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取用户的歌单
pub async fn api_playlist_user(
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let params = json!({ "plat": 1, "userid": userid, "token": token });
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({
    "userid": userid,
    "token": token,
    "total_ver": 979,
    "type": 2,
    "page": page.unwrap_or(1),
    "pagesize": page_size.unwrap_or(10),
  });

  let opts = RequestOptions::new()
    .url("/v7/get_all_list")
    .method(Method::POST)
    .add_header("x-router", "cloudlist.service.kugou.com")
    .params(params)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取歌单详情
///
/// ## 必选参数
/// * `ids` - global_collection_id / list_create_gid的集合
pub async fn api_playlist_detail(gids: Vec<&str>) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let ids: Vec<Value> = gids
    .iter()
    .map(|i| json!({ "global_collection_id": i }))
    .collect();

  let data = json!({
    "data": ids,
    "userid": kg_dynamic_config.cookies.userid,
    "token": kg_dynamic_config.cookies.token,
  });

  let opts = RequestOptions::new()
    .url("/v3/get_list_info")
    .method(Method::POST)
    .add_header("x-router", "pubsongs.kugou.com")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 收藏歌单或新建歌单
///
/// ### 说明
/// 登录状态下可收藏已有歌单或创建新歌单。
///
/// 收藏成功后，建议使用 `/playlist/tracks/add` 接口将原歌单下的歌曲添加到新歌单。
///
/// ### 必选参数
/// * `name` - 歌单名称
/// * `list_create_userid` - 歌单创建用户id

/// ### 可选参数
/// * `is_pri` - 是否设为隐私: `0` 为公开, `1` 为隐私, 默认为 `0`, 该字段仅在创建歌单时有效
/// * `list_type` - 操作类型: `0` 为创建歌单，`1` 为收藏歌单, 默认为 `0`
/// * `list_create_listid` - 歌单创建列表id
/// * `list_create_gid` - 歌单创建gid
/// * `source` - 不知道什么作用
pub async fn api_playlist_add(
  name: &str,
  list_create_userid: u64,
  is_pri: Option<u64>,
  list_type: Option<u64>,
  list_create_gid: Option<u64>,
  list_create_listid: Option<u64>,
  source: Option<u64>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;
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
      json!({ "last_time": client_time, "last_area": "gztx", "userid": userid, "token": token })
    }
    _ => json!({}),
  };
  let Value::Object(params) = params else { unreachable!() };

  let data = json!({
    "userid": userid,
    "token": token,
    "name": name,
    "list_create_userid": list_create_userid,
    "is_pri": is_pri,
    "type": list_type.unwrap_or_default(),
    "list_create_listid": list_create_listid,
    "list_create_gid": list_create_gid.unwrap_or_default(),
    "source": source,
    "total_ver": 0,
    "from_shupinmv": 0,
  });

  let opts = RequestOptions::new()
    .url("/cloudlist.service/v5/add_list")
    .method(Method::POST)
    .params(params)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 取消收藏歌单/删除歌单
///
/// ### 必选参数
/// * `listid` - 歌单id
pub async fn api_playlist_del(listid: u64) -> Result<HashMap<String, Value>, String> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();
  let kg_static_config = HttpConfig::get_kg_static_config();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let client_time = Utc::now().timestamp();

  let data = json!({ "listid": listid, "total_ver": 0, "type": 1 });
  let aes_encrypted = encrypt_aes_playlist(data.to_string()).map_err(|e| e.to_string())?;

  let p_data = json!({ "aes": aes_encrypted.key, "uid": userid, "token": token });
  let p = encrypt_rsa_pad(p_data.to_string())
    .map(|p| p.to_uppercase())
    .map_err(|e| e.to_string())?;

  let params = json!({
    "clienttime": client_time,
    "key": sign_key_params(&client_time.to_string(), None, None),
    "last_area": "gztx",
    "clientver": kg_static_config.client_ver,
    "appid": kg_static_config.appid,
    "last_time": client_time,
    "p": p,
  });
  let Value::Object(params) = params else { unreachable!() };

  let opts = RequestOptions::new()
    .url("/v2/delete_list")
    .method(Method::POST)
    .add_header("x-router", "cloudlist.service.kugou.com")
    .params(params)
    .data(Value::String(aes_encrypted.str))
    .response_type(ResponseType::Bytes);

  let resp = request::<Value>(opts).await.map_err(|e| e.to_string())?;
  let Response::Bytes(resp_bytes) = resp else { unreachable!() };

  // 如果是报错返回, 不需要解密处理, 所以先尝试解析
  let resp_str = String::from_utf8_lossy(&resp_bytes);
  if resp_str.starts_with('{') {
    return serde_json::from_str::<HashMap<String, Value>>(&resp_str).map_err(|e| e.to_string());
  }

  let resp_base64 = STANDARD.encode(&resp_bytes);
  let resp_decrypted =
    decrypt_aes_playlist(&resp_base64, &aes_encrypted.key).map_err(|e| e.to_string())?;
  let resp_map = serde_json::from_str(&resp_decrypted).map_err(|e| e.to_string())?;

  Ok(resp_map)
}

#[tauri::command]
/// 获取歌单所有歌曲(旧)
///
/// # 必选参数
/// * `gid` - 歌单 global_collection_id / list_create_gid
///
/// # 可选参数
/// * `page` - 页码, 默认为 1
/// * `page_size` - 每页数量, 默认为 30
pub async fn api_playlist_tracks_all(
  gid: &str,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let page = page.unwrap_or(1);
  let page_size = page_size.unwrap_or(30);

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

  let opts = RequestOptions::new()
    .url("/pubsongs/v2/get_other_list_file_nofilt")
    .params(params);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取歌单所有歌曲(新)
///
/// # 必选参数
/// * `listid` - 歌单id
///
/// # 可选参数
/// * `page` - 页码, 默认为 1
/// * `page_size` - 每页数量, 默认为 30
pub async fn api_playlist_tracks_all_new(
  listid: u64,
  page: Option<usize>,
  page_size: Option<usize>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let data = json!({
    "listid": listid,
    "userid": kg_dynamic_config.cookies.userid,
    "area_code": 1,
    "show_relate_goods": 0,
    "pagesize": page_size.unwrap_or(10),
    "allplatform": 1,
    "show_cover": 1,
    "type": 0,
    "token": kg_dynamic_config.cookies.token,
    "page": page.unwrap_or(1),
  });

  let opts = RequestOptions::new()
    .url("/v4/get_list_all_file")
    .method(Method::POST)
    .add_header("x-router", "cloudlist.service.kugou.com")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMusicInfo {
  name: String,
  hash: String,
  album_id: Option<u64>,
  mixsongid: Option<u64>,
}

#[tauri::command]
/// 对歌单添加歌曲
/// 说明 : 调用此接口 , 可以添加歌曲到歌单 (需要登录)
///
/// ## 必选参数
/// * `list_id` - 用户的歌单id
/// * `music_list` - 需要添加的歌曲数据
pub async fn api_playlist_tracks_add(
  list_id: u64,
  music_list: Vec<ListMusicInfo>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let client_time = Utc::now().timestamp();

  let params = json!({
    "last_time": client_time,
    "last_area": "gztx",
    "userid": userid,
    "token": token,
  });
  let Value::Object(params) = params else { unreachable!() };

  let music_list = music_list
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
    .collect::<Vec<Value>>();

  let data = json!({
    "userid": userid,
    "token": token,
    "listid": list_id,
    "list_ver": 0,
    "type": 0,
    "slow_upload": 1,
    "scene": "false;null",
    "data": music_list,
  });

  let opts = RequestOptions::new()
    .url("/cloudlist.service/v6/add_song")
    .method(Method::POST)
    .params(params)
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// 删除歌单歌曲
///
/// ## 必选参数
/// * `list_id` - 用户歌单id
/// * `file_ids` - 歌单中歌曲的 fileid
pub async fn api_playlist_tracks_del(
  list_id: u64,
  file_ids: Vec<u64>,
) -> ApiResult<HashMap<String, Value>> {
  let kg_dynamic_config = HttpConfig::get_kg_dynamic_config();

  let ids: Vec<Value> = file_ids.iter().map(|i| json!({ "fileid": i })).collect();

  let userid = kg_dynamic_config.cookies.userid;
  let token = kg_dynamic_config.cookies.token;

  let data = json!({
    "listid": list_id,
    "userid": userid,
    "data": ids,
    "type": 0,
    "token": token,
    "list_ver": 0,
  });

  let opts = RequestOptions::new()
    .url("/v4/delete_songs")
    .method(Method::POST)
    .add_header("x-router", "cloudlist.service.kugou.com")
    .data(data);

  request(opts).await.map_err(|e| e.to_string())
}
