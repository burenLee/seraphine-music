use serde::Serialize;
use std::{fs, path::PathBuf};
use tauri_plugin_http::reqwest::Method;

use crate::{
  app::dir::Dir,
  http::client::Client,
  utils::{crypto::encrypt_md5, logger::LogErrExt},
};

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
    #[cfg(target_os = "windows")]
    let _ = windows::register_commands().log_err("Media::init", "初始化系统媒体控制失败");

    #[cfg(target_os = "macos")]
    let _ = macos::register_commands().log_err("Media::init", "初始化系统媒体控制失败");

    #[cfg(target_os = "linux")]
    let _ = linux::register_commands().log_err("Media::init", "初始化系统媒体控制失败");
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
    let app = Config::app();
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

            let _ = app.emit(MEDIA_COMMAND, payload);
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

/// macOS: 基于 mediaplayer crate 的 MPRemoteCommandCenter / NowPlayingInfoCenter，
/// 接入控制中心 Now Playing 与媒体键。
#[cfg(target_os = "macos")]
mod macos {
  use mediaplayer::{
    Artwork, HandlerStatus, MediaPlayerError, NowPlayingInfo, NowPlayingInfoCenter, PlaybackState,
    RemoteCommandCenter,
  };

  use crate::{
    app::config::Config,
    log_err,
    music::media::{MediaCommandEvent, MediaMusic, MEDIA_COMMAND},
    utils::logger::LogErrExt,
  };

  pub fn register_commands() -> anyhow::Result<()> {
    let app = Config::app();
    let mut command_remote = RemoteCommandCenter::shared();

    // 注册播放命令
    let play_command = command_remote.play_command();
    play_command.set_enabled(true);
    play_command.add_handler(|_| {
      let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Play);
      HandlerStatus::Success
    })?;

    // 注册暂停命令
    let pause_command = command_remote.pause_command();
    pause_command.set_enabled(true);
    pause_command.add_handler(|_| {
      let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Pause);
      HandlerStatus::Success
    })?;

    // 注册上一曲命令
    let prev_command = command_remote.previous_track_command();
    prev_command.set_enabled(true);
    prev_command.add_handler(|_| {
      let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Prev);
      HandlerStatus::Success
    })?;

    // 注册下一曲命令
    let next_command = command_remote.next_track_command();
    next_command.set_enabled(true);
    next_command.add_handler(|_| {
      let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Next);
      HandlerStatus::Success
    })?;

    Ok(())
  }

  pub fn sync_metadata(media_music: MediaMusic) -> anyhow::Result<()> {
    let MediaMusic {
      title,
      artist,
      album,
      duration,
      position,
      is_playing,
      cover,
    } = media_music;

    // 构建播放信息
    let mut info = NowPlayingInfo::new()
      .title(title)
      .artist(artist)
      .album_title(album)
      .playback_duration(duration)
      .elapsed_playback_time(position)
      .playback_rate(if is_playing { 1.0 } else { 0.0 });

    // 封面: 加载失败则跳过, 不影响文字元数据
    let artwork = cover.as_deref().and_then(|path| {
      Artwork::from_path(path)
        .log_err("sync_metadata", "加载封面失败")
        .ok()
    });

    let info_center = NowPlayingInfoCenter::default_center();
    info_center.set_now_playing_info_with_artwork(&info, artwork.as_ref())?;
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
  use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    thread,
  };

  use anyhow::anyhow;
  use dbus::{
    arg::{RefArg, Variant},
    blocking::Connection,
    Message,
  };
  use dbus_crossroads::{Crossroads, IfaceBuilder};
  use tauri::Emitter;

  use crate::{
    app::config::Config,
    music::media::{MediaCommandEvent, MediaMusic, MEDIA_COMMAND},
    utils::logger::LogErrExt,
  };

  const DBUS_NAME: &str = "org.mpris.MediaPlayer2.seraphine";
  const OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

  struct Metadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration: Option<f64>,
    cover: Option<String>,
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

  pub fn register_commands() -> anyhow::Result<()> {
    let conn = Connection::new_session()?;
    conn.request_name(DBUS_NAME, false, true, false)?;

    let state = STATE.get_or_init(|| {
      Arc::new(State {
        conn,
        metadata: Mutex::new(None),
        playback: Mutex::new(None),
      })
    });

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
        let app = Config::app();

        move |b: &mut IfaceBuilder<()>| {
          // 方法：系统媒体键 / 媒体控件调用时回推命令
          b.method("Play", (), (), |_, _, _: ()| {
            let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Play);
            Ok(())
          });
          b.method("Pause", (), (), |_, _, _: ()| {
            let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Pause);
            Ok(())
          });
          // PlayPause: 根据当前播放状态映射为播放/暂停
          b.method("PlayPause", (), (), |_, _, _: ()| {
            let is_playing = STATE
              .get()
              .and_then(|s| {
                s.playback
                  .lock()
                  .unwrap_or_else(|e| e.into_inner())
                  .as_ref()
              })
              .map(|p| p.is_playing)
              .unwrap_or(false);
            let event = if is_playing { MediaCommandEvent::Pause } else { MediaCommandEvent::Play };
            let _ = app.emit(MEDIA_COMMAND, event);
            Ok(())
          });
          b.method("Stop", (), (), |_, _, _: ()| {
            let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Other);
            Ok(())
          });
          b.method("Next", (), (), |_, _, _: ()| {
            let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Next);
            Ok(())
          });
          b.method("Previous", (), (), |_, _, _: ()| {
            let _ = app.emit(MEDIA_COMMAND, MediaCommandEvent::Prev);
            Ok(())
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
                md.and_then(|m| m.album.as_deref()),
                md.and_then(|m| m.duration),
                md.and_then(|m| m.cover.as_deref()),
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

    Ok(())
  }

  /// 构建 MPRIS 元数据字典（a{sv}）
  fn build_metadata(
    title: Option<&str>,
    artist: Option<&str>,
    album: Option<&str>,
    duration: Option<f64>,
    cover: Option<&str>,
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
    if let Some(album) = album {
      map.insert(
        "xesam:album".to_string(),
        Variant(Box::new(album.to_string()) as Box<dyn RefArg>),
      );
    }
    if let Some(duration) = duration {
      map.insert(
        "mpris:length".to_string(),
        Variant(Box::new((duration * 1_000_000.0) as i64) as Box<dyn RefArg>),
      );
    }
    // 封面: MPRIS 只接受 URI, 本地路径转 file:// 引用
    if let Some(cover) = cover {
      map.insert(
        "mpris:artUrl".to_string(),
        Variant(Box::new(format!("file://{cover}")) as Box<dyn RefArg>),
      );
    }
    map
  }

  pub fn sync_metadata(media_music: MediaMusic) -> anyhow::Result<()> {
    let state = STATE.get().ok_or(anyhow!("获取 STATE 失败"))?;

    let MediaMusic {
      title,
      artist,
      album,
      duration,
      position,
      is_playing,
      cover,
    } = media_music;

    // 空字符串与无效数值视为未提供
    let filter = |s: String| (!s.is_empty()).then_some(s);

    *state.metadata.lock().unwrap_or_else(|e| e.into_inner()) = Some(Metadata {
      title: filter(title),
      artist: filter(artist),
      album: filter(album),
      duration: (duration > 0.0).then_some(duration),
      cover,
    });
    *state.playback.lock().unwrap_or_else(|e| e.into_inner()) = Some(Playback {
      is_playing,
      position: (position > 0.0).then_some(position),
    });

    let metadata = state.metadata.lock().unwrap_or_else(|e| e.into_inner());
    let md = metadata.as_ref();

    let mut changed: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();
    changed.insert(
      "Metadata",
      Variant(Box::new(build_metadata(
        md.and_then(|m| m.title.as_deref()),
        md.and_then(|m| m.artist.as_deref()),
        md.and_then(|m| m.album.as_deref()),
        md.and_then(|m| m.duration),
        md.and_then(|m| m.cover.as_deref()),
      )) as Box<dyn RefArg>),
    );
    changed.insert(
      "PlaybackStatus",
      Variant(
        Box::new(if is_playing { "Playing".to_string() } else { "Paused".to_string() })
          as Box<dyn RefArg>,
      ),
    );
    changed.insert(
      "Position",
      Variant(Box::new((position * 1_000_000.0) as i64) as Box<dyn RefArg>),
    );
    emit_properties_changed(&state.conn, changed);

    Ok(())
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
  let url = cover.replace("{size}", "120");
  let ext = url
    .rsplit('.')
    .next()
    .filter(|e| matches!(*e, "png" | "jpg" | "jpeg" | "webp"))
    .unwrap_or("jpg");
  let path = Dir::cover().join(format!("{}.{}", encrypt_md5(cover), ext));

  if path.exists() {
    Some(path.to_string_lossy().into_owned())
  } else {
    let bytes = Client::new(url, Method::GET)
      .bytes()
      .await
      .log_err("cache_cover", "获取封面失败")
      .ok()?;

    fs::write(&path, bytes)
      .log_err("cache_cover", "写入封面失败")
      .ok()?;

    Some(path.to_string_lossy().into_owned())
  }
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

  #[cfg(target_os = "macos")]
  macos::sync_metadata(media_music).log_command("music_media_sync_metadata", "同步失败")?;

  #[cfg(target_os = "linux")]
  linux::sync_metadata(media_music).log_command("music_media_sync_metadata", "同步失败")?;

  Ok(())
}
