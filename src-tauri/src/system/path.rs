use std::{
  env, fs,
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub struct AppPath {
  current_dir: PathBuf,
  temp_dir: PathBuf,
  lyric_dir: PathBuf,
  cover_dir: PathBuf,
}

impl AppPath {
  pub fn new() -> Self {
    let current_dir = Self::get_current_dir();
    let temp_dir = Self::get_temp_dir(&current_dir);
    let lyric_dir = Self::get_lyric_dir(&temp_dir);
    let cover_dir = Self::get_cover_dir(&temp_dir);

    Self {
      current_dir,
      temp_dir,
      lyric_dir,
      cover_dir,
    }
  }

  fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| env::home_dir().unwrap_or_else(|| env::temp_dir()))
  }

  fn get_temp_dir(current_dir: &Path) -> PathBuf {
    let temp_dir = current_dir.join("Temp");

    if !temp_dir.exists() {
      if let Err(_) = fs::create_dir_all(&temp_dir) {
        return env::temp_dir();
      }
    }

    temp_dir
  }

  fn get_lyric_dir(temp_dir: &Path) -> PathBuf {
    let lyric_dir = temp_dir.join("lyrics");

    if !lyric_dir.exists() {
      if let Err(_) = fs::create_dir_all(&lyric_dir) {
        return temp_dir.to_path_buf();
      }
    }

    lyric_dir
  }

  fn get_cover_dir(temp_dir: &Path) -> PathBuf {
    let cover_dir = temp_dir.join("covers");

    if !cover_dir.exists() {
      if let Err(_) = fs::create_dir_all(&cover_dir) {
        return temp_dir.to_path_buf();
      }
    }

    cover_dir
  }

  pub fn temp_dir(&self) -> &Path {
    &self.temp_dir
  }

  pub fn lyric_dir(&self) -> &Path {
    &self.lyric_dir
  }

  pub fn cover_dir(&self) -> &Path {
    &self.cover_dir
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppPathMap {
  temp: String,
  lyric: String,
  cover: String,
}

#[tauri::command]
pub async fn system_path_all() -> Result<AppPathMap, String> {
  let app_path = AppPath::new();

  Ok(AppPathMap {
    temp: app_path.temp_dir().to_string_lossy().to_string(),
    lyric: app_path.lyric_dir().to_string_lossy().to_string(),
    cover: app_path.cover_dir().to_string_lossy().to_string(),
  })
}
