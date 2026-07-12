use tauri::{LogicalSize, Runtime, Size, Window};

#[tauri::command]
pub async fn system_setting_restore_window<R: Runtime>(window: Window<R>) -> Result<(), String> {
  window.unmaximize().map_err(|e| e.to_string())?;
  window.set_fullscreen(false).map_err(|e| e.to_string())?;
  window
    .set_size(Size::Logical(LogicalSize {
      width: 1152.0,
      height: 768.0,
    }))
    .map_err(|e| e.to_string())?;
  window.center().map_err(|e| e.to_string())?;

  Ok(())
}
