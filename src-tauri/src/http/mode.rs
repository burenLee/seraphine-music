use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use std::sync::RwLock;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::http::config::{MODE_KEY, STORE_PATH};

static HTTP_MODE: RwLock<Mode> = RwLock::new(Mode::KgLite);

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

pub struct HttpMode;

impl HttpMode {
  pub fn init(app_handle: &AppHandle) {
    if let Ok(store) = app_handle.store(STORE_PATH) {
      let mode = store
        .get(MODE_KEY)
        .and_then(|v| from_value::<Mode>(v).ok())
        .unwrap_or_default();

      if let Ok(mut http_mode) = HTTP_MODE.write() {
        *http_mode = mode;
      }
    }
  }

  pub fn get_mode() -> Mode {
    match HTTP_MODE.read() {
      Ok(http_mode) => *http_mode,
      Err(_) => Mode::default(),
    }
  }

  pub fn set_mode(app_handle: &AppHandle, mode: Mode) {
    if let Ok(mut http_mode) = HTTP_MODE.write() {
      *http_mode = mode;
    };

    if let Ok(store) = app_handle.store(STORE_PATH) {
      store.set(MODE_KEY, json!(mode));
      let _ = store.save();
    };

    println!("set mode: {:?}", mode);
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeInfo {
  label: String,
  value: Mode,
  disabled: bool,
}

#[tauri::command]
pub fn http_mode_list() -> Vec<ModeInfo> {
  vec![
    ModeInfo {
      label: String::from("KG概念版"),
      value: Mode::KgLite,
      disabled: false,
    },
    ModeInfo {
      label: String::from("KG移动版"),
      value: Mode::KgMobile,
      disabled: true,
    },
  ]
}

#[tauri::command]
pub fn http_mode_get() -> Mode {
  HttpMode::get_mode()
}

#[tauri::command]
pub fn http_mode_set(app_handle: AppHandle, mode: Mode) {
  HttpMode::set_mode(&app_handle, mode)
}
