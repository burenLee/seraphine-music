use std::sync::OnceLock;
use tauri::AppHandle;

pub const APP_STORE: &str = "app.json";
pub const MODE_KEY: &str = "mode";

// 全局 AppHandle
pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub struct AppConfig;

impl AppConfig {
  /// ## 初始化配置
  ///
  /// ### 必选参数
  /// * `app_handle` - 应用句柄
  pub fn init(app_handle: &AppHandle) {
    APP_HANDLE.get_or_init(|| app_handle.clone());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn config_constants() {
    assert_eq!(APP_STORE, "app.json");
    assert_eq!(MODE_KEY, "mode");
  }
}
