use tauri::{
  menu::{Menu, MenuItem},
  plugin::TauriPlugin,
  tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
  AppHandle, Manager, Result, Runtime,
};
use tauri_plugin_log::{log::LevelFilter, RotationStrategy, Target, TargetKind, TimezoneStrategy};

/// ## 创建托盘图标
///
/// ### 必选参数
/// * `app_handle` - 应用句柄
pub fn build_tray_icon(app_handle: &AppHandle) -> Result<TrayIcon> {
  let quit = MenuItem::with_id(app_handle, "quit", "退出", true, None::<&str>)?;
  let menu = Menu::with_items(app_handle, &[&quit])?;

  TrayIconBuilder::new()
    .icon(app_handle.default_window_icon().unwrap().clone())
    .menu(&menu)
    .show_menu_on_left_click(false)
    .on_menu_event(|app, event| match event.id.as_ref() {
      "quit" => app.exit(0),
      _ => {}
    })
    .on_tray_icon_event(|tray, event| match event {
      TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } => {
        let app = tray.app_handle();

        // 迷你播放器打开时跳过
        if app.get_webview_window("mini-player").is_some() {
          return;
        }

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.unminimize();
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      _ => {}
    })
    .build(app_handle)
}

/// ## 构建日志插件
pub fn build_plugin_log<R: Runtime>() -> TauriPlugin<R> {
  #[cfg(debug_assertions)]
  let (level_filter, targets) = (
    LevelFilter::Debug,
    [
      Target::new(TargetKind::Stdout),
      Target::new(TargetKind::LogDir {
        file_name: Some("seraphine_music.log".to_string()),
      }),
    ],
  );

  #[cfg(not(debug_assertions))]
  let (level_filter, targets) = (
    LevelFilter::Warn,
    [Target::new(TargetKind::LogDir {
      file_name: Some("seraphine_music.log".to_string()),
    })],
  );

  // 过滤第三方依赖的日志噪声，仅保留应用自身与前端(webview)日志
  const THIRD_PARTY_PREFIXES: &[&str] = &[
    "cookie_store",
    "reqwest",
    "hyper",
    "h2",
    "h3",
    "tokio",
    "mio",
    "rustls",
    "tao",
    "wry",
    "symphonia",
    "rodio",
    "cpal",
    "souvlaki",
    "lofty",
  ];

  tauri_plugin_log::Builder::new()
    .timezone_strategy(TimezoneStrategy::UseLocal)
    .max_file_size(5 * 1024 * 1024)
    .rotation_strategy(RotationStrategy::KeepSome(3))
    .filter(|meta| {
      let target = meta.target();
      !THIRD_PARTY_PREFIXES
        .iter()
        .any(|prefix| target.starts_with(prefix))
    })
    .level(level_filter)
    .targets(targets)
    .build()
}
