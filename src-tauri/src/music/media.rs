use serde::Serialize;
use std::{fs, path::PathBuf};
use tauri_plugin_http::reqwest::Method;

#[cfg(target_os = "windows")]
use crate::utils::logger::LogErrExt;
use crate::{app::dir::Dir, http::client::Client, log_err, log_warn, utils::crypto::encrypt_md5};

/// 媒体键命令事件名（Rust -> 前端）
pub const MEDIA_COMMAND: &str = "media:command";

#[derive(Debug, Clone, Serialize)]
pub enum MediaCommandEvent {
  Play,
  Pause,
  Next,
  Prev,
  Other,
}

#[derive(Debug, Default)]
pub struct MediaMusic {
  title: String,
  artist: String,
  album: String,
  duration: f64,
  position: f64,
  is_playing: bool,
  cover: Option<String>,
}

pub struct Media;

impl Media {
  /// ## 初始化系统媒体控制：
  /// - `Windows`: SMTC
  /// - `macOS`: NowPlaying / MPRemoteCommandCenter
  /// - `Linux`: MPRIS
  pub fn init() {
    if let Err(e) = {
      #[cfg(target_os = "windows")]
      windows::register_commands()
    } {
      log_err!("Media::init", e, "初始化系统媒体控制失败");
    };
  }
}

/// Windows: 基于 MediaPlayer 的 SystemMediaTransportControls（SMTC），
/// 无需窗口句柄即可接入系统媒体控件（音量弹窗 / 媒体键）。
#[cfg(target_os = "windows")]
mod windows {
  use anyhow::anyhow;
  use std::sync::OnceLock;
  use tauri::Emitter;
  use windows::{
    core::HSTRING,
    Foundation::TypedEventHandler,
    Media::{
      MediaPlaybackStatus, MediaPlaybackType, Playback::MediaPlayer,
      SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    },
    Storage::{StorageFile, Streams::RandomAccessStreamReference},
  };

  use crate::{
    app::config::Config,
    music::media::{MediaCommandEvent, MediaMusic, MEDIA_COMMAND},
  };

  /// 持有 MediaPlayer，保证其 SMTC 保持注册状态（drop 即注销）
  static MEDIA_PLAYER: OnceLock<MediaPlayer> = OnceLock::new();

  pub fn register_commands() -> anyhow::Result<()> {
    let media_player = MediaPlayer::new()?;

    // 关闭自动联动，自己接管SMTC命令
    let command_manager = media_player.CommandManager()?;
    command_manager.SetIsEnabled(false)?;

    let smtc = media_player.SystemMediaTransportControls()?;
    smtc.ButtonPressed(&TypedEventHandler::new(
      |_, args: &Option<SystemMediaTransportControlsButtonPressedEventArgs>| {
        if let Some(args) = args {
          if let Ok(btn) = args.Button() {
            let payload = match btn {
              SystemMediaTransportControlsButton::Play => MediaCommandEvent::Play,
              SystemMediaTransportControlsButton::Pause => MediaCommandEvent::Pause,
              SystemMediaTransportControlsButton::Next => MediaCommandEvent::Next,
              SystemMediaTransportControlsButton::Previous => MediaCommandEvent::Prev,
              _ => MediaCommandEvent::Other,
            };

            let _ = Config::app().emit(MEDIA_COMMAND, payload);
          }
        }

        Ok(())
      },
    ))?;
    smtc.SetIsEnabled(true)?;
    smtc.SetIsPlayEnabled(true)?;
    smtc.SetIsPauseEnabled(true)?;
    smtc.SetIsNextEnabled(true)?;
    smtc.SetIsPreviousEnabled(true)?;

    MEDIA_PLAYER
      .set(media_player)
      .map_err(|_| anyhow!("设置 MEDIA_PLAYER 失败"))?;

    Ok(())
  }

  pub fn sync_metadata(media_music: MediaMusic) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER
      .get()
      .ok_or(anyhow!("获取 MEDIA_PLAYER 失败"))?;
    let smtc = media_player.SystemMediaTransportControls()?;

    if let Ok(updater) = smtc.DisplayUpdater() {
      if let Ok(music_props) = updater.MusicProperties() {
        music_props.SetTitle(&HSTRING::from(media_music.title))?;
        music_props.SetArtist(&HSTRING::from(media_music.artist))?;
        music_props.SetAlbumTitle(&HSTRING::from(media_music.album))?;
      }

      // 封面缩略图: SMTC 只接受本地文件引用, 加载失败则跳过, 不影响文字元数据
      if let Some(cover) = &media_music.cover {
        let cover = HSTRING::from(cover.as_str());
        if let Ok(op) = StorageFile::GetFileFromPathAsync(&cover) {
          if let Ok(file) = op.get() {
            if let Ok(stream_ref) = RandomAccessStreamReference::CreateFromFile(&file) {
              let _ = updater.SetThumbnail(&stream_ref);
            }
          }
        }
      }

      updater.SetType(MediaPlaybackType::Music)?;
      updater.Update()?;
    }

    smtc.SetPlaybackStatus(if media_music.is_playing {
      MediaPlaybackStatus::Playing
    } else {
      MediaPlaybackStatus::Paused
    })?;

    Ok(())
  }
}

#[cfg(target_os = "macos")]
mod macos {
  use mediaplayer::{
    HandlerStatus, MediaPlayerError, NowPlayingInfo, NowPlayingInfoCenter, PlaybackState,
    RemoteCommandCenter,
  };

  use crate::{log_err, utils::logger::LogErrExt};

  pub fn setup() {
    if let Err(e) = register_commands() {
      log_err!("macos::setup", e, "媒体键注册失败");
    }
  }

  fn register_commands() -> Result<(), MediaPlayerError> {
    let mut command_remote = RemoteCommandCenter::shared();

    // 注册播放命令
    let play_command = command_remote.play_command();
    let app = play_command.set_enabled(true);
    play_command
      .add_handler(|_| HandlerStatus::Success)
      .log_err("register_commands", "注册播放命令失败")?;

    // 注册暂停命令
    let pause_command = command_remote.pause_command();
    pause_command.set_enabled(true);
    pause_command
      .add_handler(|_| HandlerStatus::Success)
      .log_err("register_commands", "注册暂停命令失败")?;

    // 注册上一曲命令
    let prev_command = command_remote.previous_track_command();
    prev_command.set_enabled(true);
    prev_command
      .add_handler(|_| HandlerStatus::Success)
      .log_err("register_commands", "注册上一曲命令失败")?;

    // 注册下一曲命令
    let next_command = command_remote.next_track_command();
    next_command.set_enabled(true);
    next_command
      .add_handler(|_| HandlerStatus::Success)
      .log_err("register_commands", "注册下一曲命令失败")?;

    Ok(())
  }

  fn sync_metadata(
    title: &str,
    artist: &str,
    album: &str,
    duration: f64,
    position: f64,
    is_playing: bool,
  ) -> Result<(), MediaPlayerError> {
    // 构建播放信息
    let info = NowPlayingInfo::new()
      .title(title) // 歌曲名
      .artist(artist) // 艺术家
      .album_title(album) // 专辑名
      .playback_duration(duration) // 总时长 秒
      .elapsed_playback_time(position) // 当前已播放秒数
      .playback_rate(if is_playing { 1.0 } else { 0.0 }); // 播放倍速

    let info_center = NowPlayingInfoCenter::default_center();
    info_center.set_now_playing_info(&info)?;
    info_center.set_playback_state(if is_playing {
      PlaybackState::Playing
    } else {
      PlaybackState::Paused
    });

    Ok(())
  }
}

/// Linux: 基于 dbus-crossroads 实现 MPRIS 服务（org.mpris.MediaPlayer2.seraphine），
/// 暴露给桌面环境媒体指示器（如 GNOME/KDE 的媒体控件）。
#[cfg(target_os = "linux")]
mod linux {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex, OnceLock};
  use std::thread;

  use dbus::arg::{RefArg, Variant};
  use dbus::blocking::Connection;
  use dbus::Message;
  use dbus_crossroads::{Crossroads, IfaceBuilder};
  use tauri::{AppHandle, Emitter};

  use super::MEDIA_COMMAND;

  const DBUS_NAME: &str = "org.mpris.MediaPlayer2.seraphine";
  const OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

  struct Metadata {
    title: Option<String>,
    artist: Option<String>,
    duration: Option<f64>,
  }

  struct Playback {
    is_playing: bool,
    position: Option<f64>,
  }

  struct State {
    conn: Connection,
    metadata: Mutex<Option<Metadata>>,
    playback: Mutex<Option<Playback>>,
  }

  static STATE: OnceLock<Arc<State>> = OnceLock::new();

  pub fn setup() {
    let conn = match Connection::new_session() {
      Ok(conn) => conn,
      Err(e) => {
        log::warn!("Linux MPRIS 初始化失败: 无法连接 session bus ({e})");
        return;
      }
    };
    if let Err(e) = conn.request_name(DBUS_NAME, false, true, false) {
      log::warn!("Linux MPRIS 初始化失败: 服务名被占用 ({e})");
      return;
    }

    let state = Arc::new(State {
      conn,
      metadata: Mutex::new(None),
      playback: Mutex::new(None),
    });
    let _ = STATE.set(state.clone());

    let state = state.clone();
    let app = app.clone();
    thread::spawn(move || {
      let mut cr = Crossroads::new();

      // org.mpris.MediaPlayer2 根接口
      let root_token = cr.register("org.mpris.MediaPlayer2", |b: &mut IfaceBuilder<()>| {
        b.property("CanQuit").get(|_, _| Ok(false));
        b.property("CanRaise").get(|_, _| Ok(false));
        b.property("HasTrackList").get(|_, _| Ok(false));
        b.property("Identity")
          .get(|_, _| Ok("Seraphine".to_string()));
        b.property("SupportedUriSchemes").get(|_, _| {
          Ok(vec![
            "file".to_string(),
            "http".to_string(),
            "https".to_string(),
          ])
        });
        b.property("SupportedMimeTypes").get(|_, _| {
          Ok(vec![
            "audio/mpeg".to_string(),
            "audio/mp4".to_string(),
            "audio/ogg".to_string(),
          ])
        });
      });

      // org.mpris.MediaPlayer2.Player 接口
      let player_token = cr.register("org.mpris.MediaPlayer2.Player", {
        let state = state.clone();
        let app = app.clone();
        move |b: &mut IfaceBuilder<()>| {
          // 方法：系统媒体键 / 媒体控件调用时回推命令
          for method in ["Play", "Pause", "PlayPause", "Stop"] {
            b.method(method, (), (), {
              let handle = app.clone();
              move |_, _, _: ()| {
                let _ = handle.emit(MEDIA_COMMAND, "play_pause");
                Ok(())
              }
            });
          }
          b.method("Next", (), (), {
            let handle = app.clone();
            move |_, _, _: ()| {
              let _ = handle.emit(MEDIA_COMMAND, "next");
              Ok(())
            }
          });
          b.method("Previous", (), (), {
            let handle = app.clone();
            move |_, _, _: ()| {
              let _ = handle.emit(MEDIA_COMMAND, "prev");
              Ok(())
            }
          });
          // 拖动进度条（前端暂不处理，仅确认收到）
          b.method("Seek", ("offset",), (), |_, _, (_offset,): (i64,)| Ok(()));
          b.method("SetPosition", ("track_id", "position"), (), {
            |_, _, (_track_id, _position): (String, i64)| Ok(())
          });

          // 属性
          b.property("PlaybackStatus").get({
            let state = state.clone();
            move |_, _| {
              let playback = state.playback.lock().unwrap_or_else(|e| e.into_inner());
              let playing = playback.as_ref().map(|p| p.is_playing).unwrap_or(false);
              Ok(if playing { "Playing".to_string() } else { "Paused".to_string() })
            }
          });
          b.property("Metadata").get({
            let state = state.clone();
            move |_, _| {
              let metadata = state.metadata.lock().unwrap_or_else(|e| e.into_inner());
              let md = metadata.as_ref();
              Ok(build_metadata(
                md.and_then(|m| m.title.as_deref()),
                md.and_then(|m| m.artist.as_deref()),
                md.and_then(|m| m.duration),
              ))
            }
          });
          b.property("Position").get({
            let state = state.clone();
            move |_, _| {
              let playback = state.playback.lock().unwrap_or_else(|e| e.into_inner());
              Ok(
                playback
                  .as_ref()
                  .and_then(|p| p.position)
                  .map(|p| (p * 1_000_000.0) as i64)
                  .unwrap_or(0),
              )
            }
          });
          b.property("CanGoNext").get(|_, _| Ok(true));
          b.property("CanGoPrevious").get(|_, _| Ok(true));
          b.property("CanPlay").get(|_, _| Ok(true));
          b.property("CanPause").get(|_, _| Ok(true));
          b.property("CanSeek").get(|_, _| Ok(true));
          b.property("CanControl").get(|_, _| Ok(true));
          b.property("LoopStatus").get(|_, _| Ok("None".to_string()));
          b.property("Rate").get(|_, _| Ok(1.0_f64));
          b.property("Shuffle").get(|_, _| Ok(false));
          b.property("Volume").get(|_, _| Ok(1.0_f64));
          b.property("MinimumRate").get(|_, _| Ok(1.0_f64));
          b.property("MaximumRate").get(|_, _| Ok(1.0_f64));
        }
      });

      cr.insert(OBJECT_PATH, &[root_token, player_token], ());
      if let Err(e) = cr.serve(&state.conn) {
        log::warn!("Linux MPRIS 服务运行失败: {e}");
      }
    });
  }

  /// 构建 MPRIS 元数据字典（a{sv}）
  fn build_metadata(
    title: Option<&str>,
    artist: Option<&str>,
    duration: Option<f64>,
  ) -> HashMap<String, Variant<Box<dyn RefArg>>> {
    let mut map: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();
    map.insert(
      "mpris:trackid".to_string(),
      Variant(Box::new("/org/mpris/MediaPlayer2/Track/1".to_string()) as Box<dyn RefArg>),
    );
    if let Some(title) = title {
      map.insert(
        "xesam:title".to_string(),
        Variant(Box::new(title.to_string()) as Box<dyn RefArg>),
      );
    }
    if let Some(artist) = artist {
      map.insert(
        "xesam:artist".to_string(),
        Variant(Box::new(vec![artist.to_string()]) as Box<dyn RefArg>),
      );
    }
    if let Some(duration) = duration {
      map.insert(
        "mpris:length".to_string(),
        Variant(Box::new((duration * 1_000_000.0) as i64) as Box<dyn RefArg>),
      );
    }
    map
  }

  pub fn sync_metadata(title: Option<String>, artist: Option<String>, duration: Option<f64>) {
    let Some(state) = STATE.get() else {
      return;
    };

    *state.metadata.lock().unwrap_or_else(|e| e.into_inner()) = Some(Metadata {
      title: title.clone(),
      artist: artist.clone(),
      duration,
    });

    let mut changed: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();
    changed.insert(
      "Metadata",
      Variant(Box::new(build_metadata(
        title.as_deref(),
        artist.as_deref(),
        duration,
      )) as Box<dyn RefArg>),
    );
    emit_properties_changed(&state.conn, changed);
  }

  pub fn sync_playback(position: Option<f64>, is_playing: bool) {
    let Some(state) = STATE.get() else {
      return;
    };

    *state.playback.lock().unwrap_or_else(|e| e.into_inner()) = Some(Playback {
      is_playing,
      position,
    });

    let mut changed: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();
    changed.insert(
      "PlaybackStatus",
      Variant(
        Box::new(if is_playing { "Playing".to_string() } else { "Paused".to_string() })
          as Box<dyn RefArg>,
      ),
    );
    changed.insert(
      "Position",
      Variant(Box::new(position.map(|p| (p * 1_000_000.0) as i64).unwrap_or(0)) as Box<dyn RefArg>),
    );
    emit_properties_changed(&state.conn, changed);
  }

  /// 广播 MPRIS 属性变化，通知桌面环境媒体指示器刷新
  fn emit_properties_changed(conn: &Connection, changed: HashMap<&str, Variant<Box<dyn RefArg>>>) {
    let msg = Message::signal(
      &dbus::Path::from(OBJECT_PATH),
      &"org.freedesktop.DBus.Properties".into(),
      &"PropertiesChanged".into(),
    )
    .append1("org.mpris.MediaPlayer2.Player")
    .append2(changed, Vec::<String>::new());
    let _ = conn.send(msg);
  }
}

/// ## 获取封面本地缓存路径
///
/// 网络封面会下载并持久缓存到封面目录, 命中时直接复用
///
///
/// ### 必选参数
/// * `cover` - 封面, 本地路径或网络 URL
async fn cache_cover(cover: &str) -> Option<String> {
  // TODO: 暂时只用于媒体控件, 后续实现在前端访问网络图片缓存本地并在下次访问时读取本地图片

  // 本地路径: 存在则直接复用
  if !cover.starts_with("http://") && !cover.starts_with("https://") {
    let path = PathBuf::from(cover);
    return path.exists().then(|| cover.to_string());
  }

  // 网络 URL: 按 url 的 md5 命名, 命中复用
  let url = cover.replace("{size}", "64");
  let ext = url
    .rsplit('.')
    .next()
    .filter(|e| matches!(*e, "png" | "jpg" | "jpeg" | "webp"))
    .unwrap_or("jpg");
  let path = Dir::cover().join(format!("{}.{}", encrypt_md5(cover), ext));

  if path.exists() {
    return Some(path.to_string_lossy().into_owned());
  }

  let bytes = match Client::new(url, Method::GET).bytes().await {
    Ok(bytes) => bytes,
    Err(e) => {
      log_warn!("cache_cover", e, "下载封面失败");
      return None;
    }
  };

  if let Err(e) = fs::write(&path, &bytes) {
    log_warn!("cache_cover", e, "写入封面失败");
    return None;
  }

  Some(path.to_string_lossy().into_owned())
}

#[tauri::command]
/// ## 同步数据到媒体控制
pub async fn music_media_sync_metadata(
  cover: Option<String>,
  title: Option<String>,
  artist: Option<String>,
  album: Option<String>,
  duration: Option<f64>,
  position: Option<f64>,
  is_playing: Option<bool>,
) -> Result<(), String> {
  let mut media_music = MediaMusic::default();

  if let Some(cover) = cover {
    media_music.cover = cache_cover(&cover).await;
  }
  if let Some(title) = title {
    media_music.title = title;
  }
  if let Some(artist) = artist {
    media_music.artist = artist;
  }
  if let Some(album) = album {
    media_music.album = album;
  }
  if let Some(duration) = duration {
    media_music.duration = duration;
  }
  if let Some(position) = position {
    media_music.position = position;
  }
  if let Some(is_playing) = is_playing {
    media_music.is_playing = is_playing;
  }

  #[cfg(target_os = "windows")]
  windows::sync_metadata(media_music).log_command("music_media_sync_metadata", "同步失败")?;

  Ok(())
}
