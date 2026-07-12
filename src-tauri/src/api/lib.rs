use serde::{Deserialize, Serialize};

use crate::http::server::Response;

// 只保留需要的字段
// 考虑到未来会使用其他字段, 只删除同级字段, 不越级移动字段
// 字段可能有改动, 如果遇到 `解码失败` 的报错, 使用 hashmap 接收全部字段后检查

// 关于 ApiResponse 的 error_code 值的猜测
// 20006: 签名或者加密有问题, 20010: 少传了用户信息的某些字段

/// 统一 api 类型的 command 函数的返回类型
pub type ApiResult<T> = Result<Response<T>, String>;

/// kg api接口的一般返回结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
  pub status: i32,
  pub error_code: Option<i32>,
  pub data: Option<T>,
}

/* ---------- artist start ---------- */

pub type ArtistList = ApiResponse<ArtistListData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistListData {
  pub info: Vec<ArtistListInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistListInfo {
  pub singerid: u64,
  pub singername: String,
  pub imgurl: String,
}

/* ---------- artist end ---------- */

/* ---------- images start ---------- */
/* ---------- images end ---------- */

/* ---------- login start ---------- */

pub type LoginQrKey = ApiResponse<LoginQrKeyData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginQrKeyData {
  pub qrcode: String,
  pub qrcode_img: String,
}

pub type LoginQrCheck = ApiResponse<LoginQrCheckData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginQrCheckData {
  pub status: i32,
  pub userid: Option<u64>,
  pub nickname: Option<String>,
  pub pic: Option<String>,
  pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WxToken {
  pub access_token: String,
  pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WxTicket {
  pub errcode: i32,
  pub errmsg: String,
  pub ticket: String,
  pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WxConnect {
  pub errcode: i32,
  pub uuid: String,
  pub appname: String,
  pub qrcode: WxQrcode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WxQrcode {
  #[serde(default)]
  pub qrcodeurl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginWxCheck {
  pub wx_errcode: i32,
  pub wx_code: String,
}

pub type LoginOpenplat = ApiResponse<LoginOpenplatData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginOpenplatData {
  pub userid: u64,
  pub nickname: String,
  pub pic: String,
  pub t1: String,
  pub vip_type: u64,
  pub vip_token: String,
  pub secu_params: String,
  pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpAccessToken {
  pub access_token: String,
  pub openid: String,
  pub refresh_token: String,
  pub expires_in: u64,
  pub scope: String,
  pub unionid: String,
}

pub type LoginCaptcha = ApiResponse<LoginCaptchaData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCaptchaData {
  pub count: u64,
}

pub type LoginCellphone = ApiResponse<LoginCellphoneData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCellphoneData {
  pub userid: u64,
  pub nickname: String,
  pub pic: String,
  pub t1: String,
  pub vip_type: u64,
  pub vip_token: String,
  pub secu_params: String,
  pub token: Option<String>,
}

/* ---------- login end ---------- */

/* ---------- lyric start ---------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct LyricSearch {
  pub status: i32,
  pub candidates: Vec<LyricSearchCandidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LyricSearchCandidate {
  pub id: String,
  pub accesskey: String,
  pub product_from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LyricGet {
  pub status: i32,
  pub id: String,
  pub fmt: String,
  pub content: String,
  pub contenttype: i32,
}

/* ---------- lyric end ---------- */

/* ---------- music start ---------- */
/* ---------- music end ---------- */

/* ---------- personal start ---------- */
/* ---------- personal end ---------- */

/* ---------- playlist start ---------- */

pub type PlaylistTags = ApiResponse<Vec<PlaylistTagsData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTagsData {
  pub parent_id: String,
  pub tag_id: String,
  pub tag_name: String,
  pub sort: String,
  pub son: Option<Vec<PlaylistTagsData>>,
}

/* ---------- playlist end ---------- */

/* ---------- rank start ---------- */

pub type RankTop = ApiResponse<RankTopData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RankTopData {
  pub list: Vec<RankTopItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankTopItem {
  pub rankid: u64,
  pub rankname: String,
  pub imgurl: String,
  pub intro: String,
}

pub type RanlAudio = ApiResponse<RanlAudioData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RanlAudioData {
  pub songlist: Vec<RanlAudioSong>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RanlAudioSong {
  pub audio_id: u64,
  pub songname: String,
  pub author_name: String,
  pub album_id: u64,
  pub album_info: RanlAudioAlbumInfo,
  pub audio_info: RanlAudioInfo,
  pub business: RankAudioBusiness,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RanlAudioAlbumInfo {
  pub album_name: String,
  pub sizable_cover: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RanlAudioInfo {
  pub bitrate: u64,
  pub bitrate_flac: u64,
  pub bitrate_high: u64,
  pub bitrate_super: u64,
  pub duration_128: u64,
  pub duration_320: u64,
  pub duration_flac: u64,
  pub duration_high: u64,
  pub duration_super: u64,
  pub hash_128: String,
  pub hash_320: String,
  pub hash_flac: String,
  pub hash_high: String,
  pub hash_super: String,
  pub filesize_128: u64,
  pub filesize_320: u64,
  pub filesize_flac: u64,
  pub filesize_high: u64,
  pub filesize_super: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankAudioBusiness {
  pub original_index: u64,
}

/* ---------- rank end ---------- */

/* ---------- song start ---------- */

pub type RegisterDev = ApiResponse<RegisterDevData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDevData {
  pub dfid: String,
}

/* ---------- rank end ---------- */

/* ---------- song start ---------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct SongUrl {
  pub status: i32,
  pub url: Vec<String>,
}

/* ---------- song end ---------- */

/* ---------- top start ---------- */

pub type TopCard = ApiResponse<TopCardData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopCardData {
  pub rec_desc: String,
  pub song_list: Vec<TopCardSong>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopCardSong {
  pub songname: String,
  pub author_name: String,
  pub album_id: u64,
  pub album_name: String,
  pub sizable_cover: String,
  pub time_length: f64,
  pub hash_128: String,
  pub hash_192: String,
  pub hash_320: String,
  pub hash_ape: String,
  pub hash_flac: String,
  pub filesize_128: u64,
  pub filesize_192: u64,
  pub filesize_320: u64,
  pub filesize_ape: u64,
  pub filesize_flac: u64,
}

pub type TopPlaylist = ApiResponse<TopPlaylistData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPlaylistData {
  // 有时候不会返回这个字段
  pub special_list: Option<Vec<TopPlaylistSpecial>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPlaylistSpecial {
  pub specialid: u64,
  pub specialname: String,
  pub nickname: String,
  pub flexible_cover: String,
}

pub type TopAlbum = ApiResponse<TopAlbumData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopAlbumData {
  pub chn: Vec<TopAlbumItem>,
  pub eur: Vec<TopAlbumItem>,
  pub jpn: Vec<TopAlbumItem>,
  pub kor: Vec<TopAlbumItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopAlbumItem {
  pub albumid: u64,
  pub albumname: String,
  pub imgurl: String,
  pub singername: String,
}

/* ---------- top end ---------- */

/* ---------- user start ---------- */

pub type PlaylistUser = ApiResponse<PlaylistUserData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistUserData {
  pub info: Vec<PlaylistUserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistUserInfo {
  pub count: u64,
  pub name: String,
  pub pic: String,
  pub list_create_gid: String,
  pub list_create_listid: u64,
  pub list_create_userid: u64,
  pub list_create_username: String,
  pub musiclib_tags: Vec<PlaylistUserInfoTag>,
  pub sort: u64,
  #[serde(default)]
  pub is_def: u64, // 是否是默认歌单, 0: 不是, 1: 默认收藏, 2: 我喜欢
  #[serde(rename = "type")]
  pub list_type: u64, // 列表类型(不确定), 0: 自定义歌单, 1: 收藏歌单
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistUserInfoTag {
  pub parent_id: u64,
  pub tag_id: u64,
  pub tag_name: String,
}

pub type PlaylistTracksAll = ApiResponse<PlaylistTracksAllData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksAllData {
  begin_idx: u64,
  pagesize: u64,
  count: u64,
  list_info: PlaylistUserInfo,
  songs: Vec<PlaylistTracksAllSong>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksAllSong {
  album_id: String,
  albuminfo: PlaylistTracksAlbumInfo,
  collecttime: u64,
  cover: String,
  fileid: u64,
  language: String,
  name: String,
  relate_goods: Vec<PlaylistTracksRelateGoods>,
  singerinfo: Vec<PlaylistTracksSingerInfo>,
  timelen: u64, // 歌曲时长(ms)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksAlbumInfo {
  id: u64,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksRelateGoods {
  bitrate: u64,
  hash: String,
  level: u64,
  privilege: u64,
  size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksSingerInfo {
  avatar: String,
  id: u64,
  name: String,
}

/* ---------- user end ---------- */
