// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// ## 应用入口
fn main() {
  seraphine_music_lib::run()
}
