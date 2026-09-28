use tauri::Manager;

use crate::plugins::{build_plugin_log, build_tray_icon};

mod api;
mod app;
mod http;
mod music;
mod plugins;
mod utils;

/// ## 启动应用
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_global_shortcut::Builder::default().build())
    .plugin(tauri_plugin_autostart::Builder::default().build())
    .plugin(tauri_plugin_updater::Builder::default().build())
    .plugin(build_plugin_log())
    .setup(|app| {
      let app_handle = app.app_handle();

      build_tray_icon(app_handle)?;

      // AppConfig -> AppMode -> 其他
      // 后续的模块初始化一定要按照这个顺序
      app::config::AppConfig::init(app_handle);
      app::mode::AppMode::init();
      app::dir::AppDir::init();
      http::config::HttpConfig::init();
      http::cookie::HttpCookie::init();
      music::player::MusicPlayer::init();

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      app::mode::app_mode_all,
      app::mode::app_mode_get,
      app::mode::app_mode_set,
      app::dir::app_dirs_all,
      app::dir::app_dirs_clear,
      music::scan::music_scan_dir,
      music::scan::music_scan_types,
      music::scan::music_scan_file,
      music::scan::music_scan_cancel,
      music::file::music_file_detail,
      music::player::music_player_get_device,
      music::player::music_player_set_device,
      music::player::music_player_get_devices,
      music::player::music_player_load_file,
      music::player::music_player_load_url,
      music::player::music_player_monitor_download,
      music::player::music_player_monitor_play,
      music::player::music_player_play,
      music::player::music_player_pause,
      music::player::music_player_stop,
      music::player::music_player_seek,
      music::player::music_player_set_volume,
      music::lyric::music_lyric_get,
      // album::api_album_songs,
      api::lyric::api_lyric_search,
      api::lyric::api_lyric_get,
      api::music::api_music_everyday_recommend,
      api::personal::api_personal_fm,
      // top::api_top_album,
      api::top::api_top_card,
      api::top::api_top_playlist,
      // rank::api_rank_list,
      api::rank::api_rank_top,
      api::rank::api_rank_audio,
      api::register::api_register_dev,
      api::search::api_search,
      api::search::api_search_complex,
      api::song::api_song_url,
      api::artist::api_artist_list,
      api::artist::api_artist_detail,
      api::artist::api_artist_audios,
      // audio::api_audio_info,
      api::playlist::api_playlist_tags,
      api::playlist::api_playlist_user,
      api::playlist::api_playlist_detail,
      api::playlist::api_playlist_add,
      api::playlist::api_playlist_del,
      api::playlist::api_playlist_tracks_all,
      // playlist::api_playlist_tracks_all_new,
      api::playlist::api_playlist_tracks_add,
      api::playlist::api_playlist_tracks_del,
      // privilege::api_privilege_lite,
      // api_personal_fm,
      // api_images_audio,
      api::login::api_login_qr_key,
      api::login::api_login_qr_create,
      api::login::api_login_qr_check,
      api::login::api_login_wx_create,
      api::login::api_login_wx_check,
      api::login::api_login_openplat,
      api::login::api_login_captcha,
      api::login::api_login_cellphone,
      // login::api_login_token,
      // login::api_login_device,
      // login::api_login_device_kick,
      api::login::api_login_out,
      api::login::api_login_online,
      // user::api_user_detail,
      api::youth::api_youth_union_vip,
      api::youth::api_youth_day_vip,
      api::youth::api_youth_day_upgrade
    ])
    .run(tauri::generate_context!())
    .expect("运行播放器时出现错误!");
}
