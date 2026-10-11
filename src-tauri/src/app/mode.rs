use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{LazyLock, RwLock};
use tauri_plugin_store::StoreExt;

use crate::{
  app::{config::Config, APP_STORE, MODE_KEY},
  utils::logger::LogErrExt,
};

static APP_MODE: LazyLock<RwLock<AppMode>> = LazyLock::new(|| RwLock::new(AppMode::default()));

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AppMode {
  KgMobile,
  KgLite,
}

impl Default for AppMode {
  fn default() -> Self {
    AppMode::KgLite
  }
}

pub struct Mode;

impl Mode {
  /// ## 初始化应用模式
  ///
  /// 从 store 中加载已保存的应用模式
  pub fn init() {
    let mode = Self::load_mode();

    let mut app_mode = APP_MODE.write().unwrap_or_else(|e| e.into_inner());
    *app_mode = mode;
  }

  /// ## 从 store 加载 mode
  fn load_mode() -> AppMode {
    Config::app()
      .store(APP_STORE)
      .ok()
      .and_then(|store| store.get(MODE_KEY))
      .and_then(|value| serde_json::from_value::<AppMode>(value).ok())
      .unwrap_or_default()
  }

  /// ## 保存 mode 到 store
  ///
  /// ### 必选参数
  /// * `mode` - 应用模式
  fn save_mode(mode: &AppMode) -> anyhow::Result<()> {
    let store = Config::app().store(APP_STORE)?;
    store.set(MODE_KEY, json!(mode));
    store.save()?;

    Ok(())
  }

  /// ## 获取当前应用模式
  pub fn get_mode() -> AppMode {
    let app_mode = APP_MODE.read().unwrap_or_else(|e| e.into_inner());

    app_mode.clone()
  }

  /// ## 设置应用模式
  ///
  /// ### 必选参数
  /// * `mode` - 应用模式
  pub fn set_mode(mode: AppMode) -> anyhow::Result<()> {
    Self::save_mode(&mode)?;

    let mut app_mode = APP_MODE.write().unwrap_or_else(|e| e.into_inner());
    *app_mode = mode;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeInfo {
  label: String,
  value: AppMode,
  disabled: bool,
}

#[tauri::command]
/// ## 获取所有应用模式
pub fn app_mode_all() -> Vec<ModeInfo> {
  vec![
    ModeInfo {
      label: "KG概念版".to_string(),
      value: AppMode::KgLite,
      disabled: false,
    },
    ModeInfo {
      label: "KG移动版".to_string(),
      value: AppMode::KgMobile,
      disabled: true,
    },
  ]
}

#[tauri::command]
/// ## 获取当前应用模式
pub fn app_mode_get() -> AppMode {
  Mode::get_mode()
}

#[tauri::command]
/// ## 设置应用模式
///
/// ### 必选参数
/// * `mode` - 应用模式
pub fn app_mode_set(mode: AppMode) -> Result<(), String> {
  Mode::set_mode(mode).log_command("app_mode_set", "保存应用模式失败")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mode_default_is_kg_lite() {
    assert!(matches!(AppMode::default(), AppMode::KgLite));
  }

  #[test]
  fn mode_serializes_variant_name() {
    assert_eq!(
      serde_json::to_string(&AppMode::KgMobile).unwrap(),
      "\"KgMobile\""
    );
    assert_eq!(
      serde_json::to_string(&AppMode::KgLite).unwrap(),
      "\"KgLite\""
    );
  }

  #[test]
  fn mode_serde_roundtrip() {
    for mode in [AppMode::KgMobile, AppMode::KgLite] {
      let json = serde_json::to_string(&mode).unwrap();
      let back: AppMode = serde_json::from_str(&json).unwrap();

      assert_eq!(serde_json::to_string(&back).unwrap(), json);
    }
  }

  #[test]
  fn app_mode_all_returns_two_modes() {
    let modes = app_mode_all();

    assert_eq!(modes.len(), 2);

    assert_eq!(modes[0].label, "KG概念版");
    assert!(matches!(modes[0].value, AppMode::KgLite));
    assert!(!modes[0].disabled);

    assert_eq!(modes[1].label, "KG移动版");
    assert!(matches!(modes[1].value, AppMode::KgMobile));
    assert!(modes[1].disabled);
  }
}
