use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
  AppHandle, Manager, Result,
};

use crate::{
  api::{
    album, artist, login, lyric as api_lyric, music as api_music, personal, playlist, privilege,
    rank, register, search, song, top, user, youth,
  },
  http::{config, mode},
  music::{file, lyric as music_lyric, player, scan},
  system::{path, setting},
};

mod api;
mod http;
mod music;
mod system;
mod utils;

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_autostart::Builder::new().build())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      let app_handle = app.app_handle();

      create_tray_icon(&app_handle)?;

      // HttpMode 需要比 HttpConfig 先初始化
      mode::HttpMode::init(&app_handle);
      config::HttpConfig::init(&app_handle);

      let player = player::Player::new(app_handle)?;

      app.manage(player);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      setting::system_setting_restore_window,
      path::system_path_all,
      scan::music_scan_dir,
      scan::music_scan_type,
      scan::music_scan_file,
      scan::music_scan_cancel,
      file::music_file_detail,
      file::music_file_open,
      file::music_dir_open,
      file::music_file_clear,
      player::music_player_get_device,
      player::music_player_set_device,
      player::music_player_get_devices,
      player::music_player_load_file,
      player::music_player_load_url,
      player::music_player_monitor_download,
      player::music_player_monitor_play,
      player::music_player_play,
      player::music_player_pause,
      player::music_player_stop,
      player::music_player_seek,
      player::music_player_set_volume,
      music_lyric::music_lyric_get,
      mode::http_mode_list,
      mode::http_mode_get,
      mode::http_mode_set,
      api_lyric::api_lyric_search,
      api_lyric::api_lyric_get,
      api_music::api_music_everyday,
      personal::api_personal_fm,
      top::api_top_album,
      top::api_top_card,
      top::api_top_playlist,
      rank::api_rank_list,
      rank::api_rank_top,
      rank::api_rank_audio,
      register::api_register_dev,
      search::api_search,
      search::api_search_complex,
      song::api_song_url,
      album::api_album_songs,
      artist::api_artist_list,
      artist::api_artist_audios,
      playlist::api_playlist_tags,
      playlist::api_playlist_user,
      playlist::api_playlist_detail,
      playlist::api_playlist_add,
      playlist::api_playlist_del,
      playlist::api_playlist_tracks_all,
      playlist::api_playlist_tracks_all_new,
      playlist::api_playlist_tracks_add,
      playlist::api_playlist_tracks_del,
      privilege::api_privilege_lite,
      // api_personal_fm,
      // api_images_audio,
      login::api_login_qr_key,
      login::api_login_qr_create,
      login::api_login_qr_check,
      login::api_login_wx_create,
      login::api_login_wx_check,
      login::api_login_openplat,
      login::api_login_captcha,
      login::api_login_cellphone,
      login::api_login_token,
      login::api_login_device,
      login::api_login_device_kick,
      login::api_login_out,
      user::api_user_detail,
      youth::api_youth_union_vip,
      youth::api_youth_day_vip,
      youth::api_youth_day_upgrade
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// 创建托盘图标
fn create_tray_icon(app_handle: &AppHandle) -> Result<TrayIcon> {
  let show = MenuItem::with_id(app_handle, "show", "显示窗口", true, None::<&str>)?;
  let quit = MenuItem::with_id(app_handle, "quit", "退出", true, None::<&str>)?;

  let menu = Menu::with_items(app_handle, &[&show, &quit])?;

  let tray = TrayIconBuilder::new()
    .icon(app_handle.default_window_icon().unwrap().clone())
    .menu(&menu)
    .show_menu_on_left_click(false)
    .on_menu_event(|app, event| match event.id.as_ref() {
      "show" => {
        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      "quit" => {
        app.exit(0);
      }
      _ => {}
    })
    .on_tray_icon_event(|tray, event| match event {
      TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } => {
        if let Some(window) = tray.app_handle().get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      _ => {}
    });

  tray.build(app_handle)
}
