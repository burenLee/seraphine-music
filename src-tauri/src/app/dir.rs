use std::{collections::HashMap, env, fs, path::PathBuf, sync::OnceLock};
use tauri::Manager;

use crate::{app::config::Config, log_err, utils::logger::LogErrExt};

const AUDIO_DIR: &str = "audios";
const LYRIC_DIR: &str = "lyrics";
const COVER_DIR: &str = "covers";

static APP_DIR: OnceLock<AppDir> = OnceLock::new();

struct AppDir {
  app_data: PathBuf,

  app_local_data: PathBuf,
  app_log: PathBuf,

  audio: PathBuf,
  lyric: PathBuf,
  cover: PathBuf,
}

pub struct Dir;

impl Dir {
  /// ## 初始化目录
  ///
  /// 创建音频 / 歌词 / 封面目录
  pub fn init() {
    let path = Config::app().path();

    let app_data_dir = path.app_data_dir().unwrap_or_else(|_| {
      path
        .app_local_data_dir()
        .unwrap_or_else(|_| env::temp_dir())
    });

    let app_local_data_dir = path
      .app_local_data_dir()
      .unwrap_or_else(|_| env::temp_dir());
    let app_log_dir = path.app_log_dir().unwrap_or_else(|_| env::temp_dir());

    let audio_dir = Self::gen_dir(&app_local_data_dir, AUDIO_DIR);
    let lyric_dir = Self::gen_dir(&app_local_data_dir, LYRIC_DIR);
    let cover_dir = Self::gen_dir(&app_local_data_dir, COVER_DIR);

    APP_DIR.get_or_init(|| AppDir {
      app_data: app_data_dir,
      app_local_data: app_local_data_dir,
      app_log: app_log_dir,
      audio: audio_dir,
      lyric: lyric_dir,
      cover: cover_dir,
    });
  }

  /// ## 生成目录
  ///
  /// ### 必选参数
  /// * `parent` - 父目录
  /// * `dir_path` - 目录名称
  fn gen_dir(parent: &PathBuf, dir_path: &str) -> PathBuf {
    let path = parent.join(dir_path);

    if !path.exists() {
      if let Err(e) = fs::create_dir_all(&path) {
        log_err!("gen_dir", e, format!("创建目录失败[{}]", path.display()));

        return parent.clone();
      }
    }

    path
  }

  /// ## 获取应用数据目录
  pub fn app_data() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.app_data.clone(),
      None => env::temp_dir(),
    }
  }

  /// ## 获取本地数据目录
  pub fn app_local_data() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.app_local_data.clone(),
      None => env::temp_dir(),
    }
  }

  /// ## 获取日志目录
  pub fn app_log() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.app_log.clone(),
      None => env::temp_dir(),
    }
  }

  /// ## 获取音频目录
  pub fn audio() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.audio.clone(),
      None => env::temp_dir(),
    }
  }

  /// ## 获取歌词目录
  pub fn lyric() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.lyric.clone(),
      None => env::temp_dir(),
    }
  }

  /// ## 获取封面目录
  pub fn cover() -> PathBuf {
    match APP_DIR.get() {
      Some(dirs) => dirs.cover.clone(),
      None => env::temp_dir(),
    }
  }
}

#[tauri::command]
/// ## 获取所有目录路径
pub fn app_dirs_all() -> HashMap<String, String> {
  HashMap::from_iter([
    (
      "app_data".to_string(),
      Dir::app_data().to_string_lossy().to_string(),
    ),
    (
      "app_local_data".to_string(),
      Dir::app_local_data().to_string_lossy().to_string(),
    ),
    (
      "app_log".to_string(),
      Dir::app_log().to_string_lossy().to_string(),
    ),
    (
      "audio".to_string(),
      Dir::audio().to_string_lossy().to_string(),
    ),
    (
      "lyric".to_string(),
      Dir::lyric().to_string_lossy().to_string(),
    ),
    (
      "cover".to_string(),
      Dir::cover().to_string_lossy().to_string(),
    ),
  ])
}

#[tauri::command]
/// ## 清理目录下的所有文件
///
/// ### 必选参数
/// * `path` - 目录路径
pub fn app_dirs_clear(path: &str) -> Result<(), String> {
  let dir_path = PathBuf::from(path);
  if !dir_path.exists() {
    return Err("目录不存在".to_string());
  }
  if !dir_path.is_dir() {
    return Err("非目录路径".to_string());
  }

  let mut fail_total = 0; // 失败数
  let entries = fs::read_dir(dir_path).log_command("app_dirs_clear", "读取目录失败")?;

  for entry in entries {
    let path = entry
      .map(|e| e.path())
      .log_command("app_dirs_clear", "获取文件路径失败")?;

    // 仅清理文件
    if path.is_file() {
      if let Err(e) = fs::remove_file(&path) {
        fail_total = fail_total + 1;

        log_err!(
          "app_dirs_clear",
          e,
          format!("清理文件[{}]失败:", path.display())
        );
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dir_constants() {
    assert_eq!(AUDIO_DIR, "audios");
    assert_eq!(LYRIC_DIR, "lyrics");
    assert_eq!(COVER_DIR, "covers");
  }

  #[test]
  fn gen_dir_creates_directory() {
    let temp = tempfile::tempdir().unwrap();
    let parent = temp.path().to_path_buf();

    let child = Dir::gen_dir(&parent, "sub");

    assert!(child.is_dir());
    assert_eq!(child, parent.join("sub"));
  }

  #[test]
  fn gen_dir_reuses_existing_directory() {
    let temp = tempfile::tempdir().unwrap();
    let parent = temp.path().to_path_buf();

    let first = Dir::gen_dir(&parent, "sub");
    let second = Dir::gen_dir(&parent, "sub");

    assert_eq!(first, second);
  }
}
