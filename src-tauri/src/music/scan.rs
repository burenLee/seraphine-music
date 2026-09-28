use anyhow::anyhow;
use lofty::{
  file::{AudioFile, TaggedFileExt},
  probe::Probe,
  tag::Accessor,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashSet,
  fs,
  path::{Path, PathBuf},
  sync::atomic::{AtomicBool, Ordering},
};
use walkdir::WalkDir;

use crate::{
  log_err,
  music::file::music_file_cover,
  utils::{crypto::encrypt_md5, logger::LogErrExt},
};

// rodio支持的音频格式
const MUSIC_EXT: [&str; 12] = [
  "flac", "mp3", "wav", "ogg", "aac", "m4a", "m4b", "aiff", "aif", "aifc", "alac", "mka",
];
// 文件夹最大深度
const MAX_DEPTH: usize = 8;

// 扫描取消标志
static SCAN_CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MusicInfo {
  id: String,             // 路径的md5值
  hash: Option<String>,   // hash, 本地音频为 Null
  path: String,           // 路径
  cover: Option<String>,  // 封面
  title: String,          // 标题
  artist: Option<String>, // 歌手
  album: Option<String>,  // 专辑
  duration: f64,          // 时长 (s)
  sort: usize,            // 排序
}

struct MusicScan;

impl MusicScan {
  /// ## 扫描文件夹
  ///
  /// ### 必选参数
  /// * `dir_paths` - 扫描文件夹集合
  /// * `start_index` - 扫描开始索引, 用于 sort 赋值
  /// * `scan_types` - 扫描类型
  ///
  /// ### 返回结果
  /// 扫描被取消时返回 `None`
  pub fn scan_dir(
    dir_paths: Vec<String>,
    start_index: usize,
    scan_types: Vec<String>,
  ) -> Option<Vec<MusicInfo>> {
    SCAN_CANCELLED.store(false, Ordering::Relaxed);

    let mut file_paths = Vec::new();

    for dir_path in Self::filter_dir_path(dir_paths) {
      for entry in WalkDir::new(dir_path).max_depth(MAX_DEPTH).into_iter() {
        if SCAN_CANCELLED.load(Ordering::Relaxed) {
          return None;
        }

        let entry = match entry {
          Ok(e) => e,
          Err(e) => {
            log_err!("scan_dir", e, "遍历目录失败");
            continue;
          }
        };

        if !entry.file_type().is_file() {
          continue;
        }

        let path = entry.into_path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
          if scan_types.iter().any(|t| t == ext) {
            file_paths.push(path);
          }
        }
      }
    }

    let mut music_list: Vec<MusicInfo> = file_paths
      .par_iter()
      .filter_map(|path| {
        if SCAN_CANCELLED.load(Ordering::Relaxed) {
          return None;
        }

        Self::gen_music(path)
          .log_err("scan_dir", "生成音频失败")
          .ok()
      })
      .collect();

    // 按扫描顺序设置排序值
    music_list
      .iter_mut()
      .zip(start_index..)
      .for_each(|(music_info, index)| {
        music_info.sort = index;
      });

    if SCAN_CANCELLED.load(Ordering::Relaxed) {
      return None;
    }

    Some(music_list)
  }

  /// ## 扫描文件
  ///
  /// ### 必选参数
  /// * `file_paths` - 扫描文件路径集合
  /// * `start_index` - 扫描开始索引, 用于 sort 赋值
  ///
  /// ### 返回结果
  /// 扫描被取消时返回 `None`
  pub fn scan_file(file_paths: Vec<String>, start_index: usize) -> Option<Vec<MusicInfo>> {
    SCAN_CANCELLED.store(false, Ordering::Relaxed);

    let file_paths = Self::filter_file_path(file_paths);

    let mut music_list: Vec<MusicInfo> = file_paths
      .par_iter()
      .filter_map(|path| {
        if SCAN_CANCELLED.load(Ordering::Relaxed) {
          return None;
        }

        Self::gen_music(path)
          .log_err("scan_file", "生成音频失败")
          .ok()
      })
      .collect();

    // 按扫描顺序设置排序值
    music_list
      .iter_mut()
      .zip(start_index..)
      .for_each(|(music_info, index)| {
        music_info.sort = index;
      });

    if SCAN_CANCELLED.load(Ordering::Relaxed) {
      return None;
    }

    Some(music_list)
  }

  /// ## 过滤目录路径
  ///
  /// 去重并剔除子目录, 仅保留最外层且合法的目录
  ///
  /// ### 必选参数
  /// * `dir_paths` - 目录路径集合
  fn filter_dir_path(dir_paths: Vec<String>) -> Vec<PathBuf> {
    let mut filter_paths = Vec::new();

    let dir_paths_set: HashSet<String> = dir_paths.into_iter().collect();
    let mut dir_paths: Vec<PathBuf> = dir_paths_set
      .into_iter()
      .map(|p| PathBuf::from(p))
      .collect();
    // 按目录深度排序
    dir_paths.sort_by_key(|path| path.components().count());

    for dir_path in dir_paths {
      let Ok(metadata) = fs::metadata(&dir_path) else {
        continue;
      };

      // 跳过 非绝对路径 / 符号链接 / 文件路径
      if !dir_path.is_absolute() || metadata.is_symlink() || metadata.is_file() {
        continue;
      };

      // 检查是否已经有父目录被添加
      let has_parent = filter_paths
        .iter()
        .any(|filter_path: &PathBuf| dir_path.starts_with(filter_path) && filter_path != &dir_path);

      if !has_parent {
        // 移除已添加的子目录
        filter_paths.retain(|filter_path: &PathBuf| {
          !filter_path.starts_with(&dir_path) || filter_path == &dir_path
        });
        filter_paths.push(dir_path);
      }
    }

    filter_paths
  }

  /// ## 过滤文件路径
  ///
  /// 去重并剔除非绝对路径 / 符号链接 / 文件夹路径
  ///
  /// ### 必选参数
  /// * `file_paths` - 文件路径集合
  fn filter_file_path(file_paths: Vec<String>) -> Vec<PathBuf> {
    let file_paths_set: HashSet<String> = file_paths.into_iter().collect();

    file_paths_set
      .into_iter()
      .filter_map(|file_path| {
        let path = PathBuf::from(file_path);

        let Ok(metadata) = fs::metadata(&path) else {
          return None;
        };

        // 跳过 非绝对路径 / 符号链接 / 文件夹路径
        if !path.is_absolute() || metadata.is_symlink() || metadata.is_dir() {
          return None;
        };

        Some(path)
      })
      .collect()
  }

  /// ## 生成音频信息
  ///
  /// 读取音频的标签信息, 存在封面时同时保存到本地
  ///
  /// ### 必选参数
  /// * `path` - 音频文件路径
  fn gen_music(path: &Path) -> anyhow::Result<MusicInfo> {
    let tagged_file = Probe::open(path)?.read()?;
    let file_path = path.to_string_lossy();

    let mut music_info = MusicInfo::default();

    music_info.id = encrypt_md5(file_path.as_ref());
    music_info.path = file_path.to_string();
    music_info.duration = tagged_file.properties().duration().as_secs_f64();

    if let Some(tag) = tagged_file.primary_tag() {
      let file_stem = path
        .file_stem()
        .map(|n| n.to_string_lossy())
        .ok_or(anyhow!("无法获取文件名称"))?;

      music_info.title = tag.title().unwrap_or(file_stem.clone()).into_owned();
      music_info.artist = tag.artist().map(|a| a.into_owned());
      music_info.album = tag.album().map(|a| a.into_owned());

      // 获取并保存封面
      if let Some(picture) = tag.pictures().get(0) {
        music_info.cover = music_file_cover(&file_stem, picture);
      }
    }

    Ok(music_info)
  }
}

#[tauri::command]
/// ## 获取支持的音频格式
pub fn music_scan_types() -> Vec<String> {
  MUSIC_EXT.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
/// ## 扫描文件夹
///
/// ### 必选参数
/// * `dir_paths` - 扫描文件夹集合
///
/// ### 可选参数
/// * `start_index` - 扫描开始索引, 用于 sort 赋值, 默认为 0
/// * `scan_types` - 扫描类型, 默认为支持的全部音频格式
///
/// ### 返回结果
/// 扫描被取消时返回 `None`
pub async fn music_scan_dir(
  dir_paths: Vec<String>,
  start_index: Option<usize>,
  scan_types: Option<Vec<String>>,
) -> Option<Vec<MusicInfo>> {
  let start_index = start_index.unwrap_or_default();
  let scan_types = scan_types.unwrap_or_else(|| MUSIC_EXT.iter().map(|e| e.to_string()).collect());

  MusicScan::scan_dir(dir_paths, start_index, scan_types)
}

#[tauri::command]
/// ## 扫描文件
///
/// ### 必选参数
/// * `file_paths` - 扫描文件路径集合
///
/// ### 可选参数
/// * `start_index` - 扫描开始索引, 用于 sort 赋值, 默认为 0
///
/// ### 返回结果
/// 扫描被取消时返回 `None`
pub async fn music_scan_file(
  file_paths: Vec<String>,
  start_index: Option<usize>,
) -> Option<Vec<MusicInfo>> {
  let start_index = start_index.unwrap_or_default();

  MusicScan::scan_file(file_paths, start_index)
}

#[tauri::command]
/// ## 取消扫描
pub fn music_scan_cancel() {
  SCAN_CANCELLED.store(true, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
  use std::{fs, path::PathBuf};

  use super::*;

  #[test]
  fn music_scan_types_returns_all_extensions() {
    let types = music_scan_types();

    assert_eq!(types.len(), MUSIC_EXT.len());
    assert!(types.contains(&"mp3".to_string()));
    assert!(types.contains(&"flac".to_string()));
  }

  #[test]
  fn music_info_serde_roundtrip() {
    let info = MusicInfo {
      id: "id".to_string(),
      hash: None,
      path: "/path".to_string(),
      cover: None,
      title: "title".to_string(),
      artist: Some("artist".to_string()),
      album: None,
      duration: 123.5,
      sort: 3,
    };

    let json = serde_json::to_string(&info).unwrap();
    let back: MusicInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(back.id, "id");
    assert_eq!(back.hash, None);
    assert_eq!(back.title, "title");
    assert_eq!(back.artist.as_deref(), Some("artist"));
    assert_eq!(back.duration, 123.5);
    assert_eq!(back.sort, 3);
  }

  #[test]
  fn filter_file_path_keeps_absolute_file_and_dedups() {
    let temp = tempfile::tempdir().unwrap();
    let file_path = temp.path().join("a.mp3");
    fs::write(&file_path, b"x").unwrap();
    let abs = file_path.to_string_lossy().to_string();

    let result = MusicScan::filter_file_path(vec![abs.clone(), abs]);

    assert_eq!(result, vec![file_path]);
  }

  #[test]
  fn filter_file_path_rejects_relative_and_nonexistent() {
    let result = MusicScan::filter_file_path(vec![
      "relative/path.mp3".to_string(),
      "/nonexistent/abs.mp3".to_string(),
    ]);

    assert!(result.is_empty());
  }

  #[test]
  fn filter_dir_path_removes_subdir_when_parent_present() {
    let temp = tempfile::tempdir().unwrap();
    let parent = temp.path().to_path_buf();
    let sub = parent.join("sub");
    fs::create_dir_all(&sub).unwrap();

    let result = MusicScan::filter_dir_path(vec![
      parent.to_string_lossy().to_string(),
      sub.to_string_lossy().to_string(),
    ]);

    assert_eq!(result, vec![parent]);
  }

  #[test]
  fn filter_dir_path_rejects_relative() {
    let result = MusicScan::filter_dir_path(vec!["relative/dir".to_string()]);

    assert!(result.is_empty());
  }
}
