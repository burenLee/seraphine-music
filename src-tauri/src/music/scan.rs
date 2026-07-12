use anyhow::anyhow;
use lofty::{
  file::{AudioFile, TaggedFileExt},
  probe::Probe,
  tag::Accessor,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::{
  collections::HashSet,
  fs::{metadata, read_dir},
  path::{Path, PathBuf},
  sync::atomic::{AtomicBool, Ordering},
};

use crate::{music::file::music_file_cover, system::path::AppPath, utils::crypto::encrypt_md5};

// rodio支持的音频格式
const MUSIC_EXT: [&str; 12] = [
  "flac", "mp3", "wav", "ogg", "aac", "m4a", "m4b", "aiff", "aif", "aifc", "alac", "mka",
];
// 文件夹最大深度
const MAX_DEPTH: usize = 8;

// 扫描取消标志
static SCAN_CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default, Serialize)]
pub struct ListMusic {
  id: String,             // 路径的md5值
  hash: Option<String>,   // hash, 本地音频一般为 Null
  path: String,           // 路径
  cover: Option<String>,  // 封面
  title: String,          // 标题
  artist: Option<String>, // 歌手
  album: Option<String>,  // 专辑
  duration: f64,          // 时长 (s)
  sort: usize,            // 排序
}

struct MusicScan {
  app_path: AppPath,
  scan_types: Vec<String>,
}

impl MusicScan {
  pub fn new(scan_types: Vec<String>) -> Self {
    Self {
      app_path: AppPath::new(),
      scan_types,
    }
  }

  /// 扫描文件夹
  pub fn scan_dir(&mut self, dir_paths: Vec<String>, start_index: usize) -> Option<Vec<ListMusic>> {
    SCAN_CANCELLED.store(false, Ordering::Relaxed);

    let mut music_list = Vec::new();

    self
      .filter_dir_path(dir_paths)
      .into_iter()
      .for_each(|path| {
        let depth = path.components().count();

        if let Err(e) = self.scan_recursion(&mut music_list, &path, depth) {
          eprintln!("{e}")
        }
      });

    if SCAN_CANCELLED.load(Ordering::Relaxed) {
      return None;
    }

    // 按扫描顺序设置排序值
    music_list
      .iter_mut()
      .zip(start_index..)
      .for_each(|(music_info, index)| {
        music_info.sort = index;
      });

    Some(music_list)
  }

  /// 扫描文件
  pub fn scan_file(
    &mut self,
    file_paths: Vec<String>,
    start_index: usize,
  ) -> Option<Vec<ListMusic>> {
    SCAN_CANCELLED.store(false, Ordering::Relaxed);

    let mut music_list = Vec::new();

    for (index, file_path) in self.filter_file_path(file_paths).into_iter().enumerate() {
      if SCAN_CANCELLED.load(Ordering::Relaxed) {
        return None;
      }

      match self.gen_music(&file_path) {
        Ok(mut music_info) => {
          music_info.sort = start_index + index;
          music_list.push(music_info);
        }
        Err(e) => eprintln!("{e}"),
      }
    }

    Some(music_list)
  }

  /// 过滤目录路径
  fn filter_dir_path(&self, dir_paths: Vec<String>) -> Vec<PathBuf> {
    let mut filter_paths = Vec::new();

    let dir_paths_set: HashSet<String> = dir_paths.into_iter().collect();
    let mut dir_paths: Vec<PathBuf> = dir_paths_set
      .into_iter()
      .map(|p| PathBuf::from(p))
      .collect();
    // 按目录深度排序
    dir_paths.sort_by_key(|path| path.components().count());

    for dir_path in dir_paths {
      let Ok(metadata) = metadata(&dir_path) else {
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

  /// 过滤文件路径
  fn filter_file_path(&self, file_paths: Vec<String>) -> Vec<PathBuf> {
    let file_paths_set: HashSet<String> = file_paths.into_iter().collect();

    file_paths_set
      .into_iter()
      .filter_map(|file_path| {
        let path = PathBuf::from(file_path);

        let Ok(metadata) = metadata(&path) else {
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

  // 递归扫描
  fn scan_recursion(
    &self,
    music_list: &mut Vec<ListMusic>,
    path: &Path,
    depth: usize,
  ) -> anyhow::Result<()> {
    // 递归深度限制
    if depth > MAX_DEPTH {
      return Ok(());
    }

    // 跳过以'.'开头的目录
    if path
      .file_name()
      .and_then(|n| n.to_str())
      .map_or(false, |n| n.starts_with('.'))
    {
      return Ok(());
    }

    let mut dir_paths = Vec::new();
    let mut file_paths = Vec::new();

    for entry in read_dir(path)? {
      if SCAN_CANCELLED.load(Ordering::Relaxed) {
        return Ok(());
      }

      let path = entry?.path();
      let metadata = metadata(&path)?;

      if metadata.is_symlink() {
        // 跳过符号链接
        continue;
      } else if metadata.is_dir() {
        dir_paths.push(path);
      } else if metadata.is_file() {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
          if self.scan_types.iter().any(|t| t == ext) {
            file_paths.push(path);
          }
        }
      }
    }

    let mut list: Vec<ListMusic> = file_paths
      .par_iter()
      .filter_map(|path| self.gen_music(path).ok())
      .collect();
    music_list.append(&mut list);

    dir_paths.iter().for_each(|path| {
      if let Err(e) = self.scan_recursion(music_list, &path, depth + 1) {
        eprintln!("{e}");
      }
    });

    Ok(())
  }

  /// 生成音频信息
  fn gen_music(&self, path: &Path) -> anyhow::Result<ListMusic> {
    let mut music_info = ListMusic::default();

    let file_path = path.to_string_lossy();
    let tagged_file = Probe::open(&path)?.read()?;

    music_info.id = encrypt_md5(file_path.as_ref());
    music_info.hash = None;
    music_info.path = file_path.to_string();
    music_info.duration = tagged_file.properties().duration().as_secs_f64();

    if let Some(tag) = tagged_file.primary_tag() {
      let file_stem = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or(anyhow!("无法获取文件名: {}", file_path))?;

      let mut title = tag.title().map(|t| t.into_owned());
      let mut artist = tag.artist().map(|t| t.into_owned());

      if title.is_none() || artist.is_none() {
        // 尝试从文件名中获取标题和艺术家
        let file_stem_slice = file_stem.split_once('-');

        if let Some((stem_title, stem_artist)) = file_stem_slice {
          title = Some(stem_title.trim().to_owned());
          artist = Some(stem_artist.trim().to_owned());
        } else {
          title = Some(file_stem.to_owned());
        }
      }

      if title.is_none() {
        return Err(anyhow!("无法获取文件名称"));
      }

      music_info.title = title.unwrap();
      music_info.artist = artist;
      music_info.album = tag.album().map(|a| a.into_owned());

      // 获取并保存封面
      if let Some(pic) = tag.pictures().get(0) {
        music_info.cover = music_file_cover(self.app_path.cover_dir(), file_stem, pic);
      }
    }

    Ok(music_info)
  }
}

#[tauri::command]
/// 获取支持的音频格式
pub fn music_scan_type() -> Vec<String> {
  MUSIC_EXT.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
/// 扫描文件夹
pub async fn music_scan_dir(
  dir_paths: Vec<String>,
  start_index: Option<usize>,
  scan_types: Option<Vec<String>>,
) -> Option<Vec<ListMusic>> {
  let scan_types = scan_types.unwrap_or_else(|| MUSIC_EXT.iter().map(|e| e.to_string()).collect());
  let mut music_scan = MusicScan::new(scan_types);

  music_scan.scan_dir(dir_paths, start_index.unwrap_or_default())
}

#[tauri::command]
/// 扫描文件
pub async fn music_scan_file(
  file_paths: Vec<String>,
  start_index: Option<usize>,
  scan_types: Option<Vec<String>>,
) -> Option<Vec<ListMusic>> {
  let scan_types = scan_types.unwrap_or_else(|| MUSIC_EXT.iter().map(|e| e.to_string()).collect());
  let mut music_scan = MusicScan::new(scan_types);

  music_scan.scan_file(file_paths, start_index.unwrap_or_default())
}

#[tauri::command]
/// 取消扫描
pub fn music_scan_cancel() {
  SCAN_CANCELLED.store(true, Ordering::Relaxed);
}
