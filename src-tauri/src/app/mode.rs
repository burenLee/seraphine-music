use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{LazyLock, RwLock};
use tauri_plugin_store::StoreExt;

use crate::{
  app::config::{APP_HANDLE, APP_STORE, MODE_KEY},
  utils::logger::LogErrExt,
};

static MODE: LazyLock<RwLock<Mode>> = LazyLock::new(|| RwLock::new(Mode::default()));

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
  KgMobile,
  KgLite,
}

impl Default for Mode {
  fn default() -> Self {
    Mode::KgLite
  }
}

pub struct AppMode;

impl AppMode {
  /// ## 初始化应用模式
  ///
  /// 从 store 中加载已保存的应用模式
  pub fn init() {
    let mode = Self::load_mode();

    let mut app_mode = MODE.write().unwrap_or_else(|e| e.into_inner());
    *app_mode = mode;
  }

  /// ## 从 store 加载 mode
  fn load_mode() -> Mode {
    APP_HANDLE
      .get()
      .and_then(|app| app.store(APP_STORE).ok())
      .and_then(|store| store.get(MODE_KEY))
      .and_then(|value| serde_json::from_value::<Mode>(value).ok())
      .unwrap_or_default()
  }

  /// ## 保存 mode 到 store
  ///
  /// ### 必选参数
  /// * `mode` - 应用模式
  fn save_mode(mode: &Mode) -> anyhow::Result<()> {
    let app_handle = APP_HANDLE.get().ok_or(anyhow!("未获取到APP_HANDLE"))?;
    let store = app_handle.store(APP_STORE)?;
    store.set(MODE_KEY, json!(mode));
    store.save()?;

    Ok(())
  }

  /// ## 获取当前应用模式
  pub fn get_mode() -> Mode {
    *MODE.read().unwrap_or_else(|e| e.into_inner())
  }

  /// ## 设置应用模式
  ///
  /// ### 必选参数
  /// * `mode` - 应用模式
  pub fn set_mode(mode: Mode) -> anyhow::Result<()> {
    Self::save_mode(&mode)?;

    let mut app_mode = MODE.write().unwrap_or_else(|e| e.into_inner());
    *app_mode = mode;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeInfo {
  label: String,
  value: Mode,
  disabled: bool,
}

#[tauri::command]
/// ## 获取所有应用模式
pub fn app_mode_all() -> Vec<ModeInfo> {
  vec![
    ModeInfo {
      label: "KG概念版".to_string(),
      value: Mode::KgLite,
      disabled: false,
    },
    ModeInfo {
      label: "KG移动版".to_string(),
      value: Mode::KgMobile,
      disabled: true,
    },
  ]
}

#[tauri::command]
/// ## 获取当前应用模式
pub fn app_mode_get() -> Mode {
  AppMode::get_mode()
}

#[tauri::command]
/// ## 设置应用模式
///
/// ### 必选参数
/// * `mode` - 应用模式
pub fn app_mode_set(mode: Mode) -> Result<(), String> {
  AppMode::set_mode(mode).log_command(module_path!(), "[app_mode_set] 保存应用模式失败")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mode_default_is_kg_lite() {
    assert!(matches!(Mode::default(), Mode::KgLite));
  }

  #[test]
  fn mode_serializes_variant_name() {
    assert_eq!(serde_json::to_string(&Mode::KgMobile).unwrap(), "\"KgMobile\"");
    assert_eq!(serde_json::to_string(&Mode::KgLite).unwrap(), "\"KgLite\"");
  }

  #[test]
  fn mode_serde_roundtrip() {
    for mode in [Mode::KgMobile, Mode::KgLite] {
      let json = serde_json::to_string(&mode).unwrap();
      let back: Mode = serde_json::from_str(&json).unwrap();

      assert_eq!(serde_json::to_string(&back).unwrap(), json);
    }
  }

  #[test]
  fn app_mode_all_returns_two_modes() {
    let modes = app_mode_all();

    assert_eq!(modes.len(), 2);

    assert_eq!(modes[0].label, "KG概念版");
    assert!(matches!(modes[0].value, Mode::KgLite));
    assert!(!modes[0].disabled);

    assert_eq!(modes[1].label, "KG移动版");
    assert!(matches!(modes[1].value, Mode::KgMobile));
    assert!(modes[1].disabled);
  }
}
