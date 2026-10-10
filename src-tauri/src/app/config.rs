use std::sync::OnceLock;
use tauri::AppHandle;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub struct Config;

impl Config {
  /// ## 初始化配置
  ///
  /// ### 必选参数
  /// * `app_handle` - 应用句柄
  pub fn init(app_handle: &AppHandle) {
    APP_HANDLE
      .set(app_handle.clone())
      .expect("无法设置 APP_HANDLE");
  }

  /// ## 获取APP_HANDLE
  pub fn app() -> &'static AppHandle {
    APP_HANDLE.get().expect("无法获取 APP_HANDLE")
  }
}
