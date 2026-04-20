use std::sync::atomic::{AtomicUsize, Ordering};

use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use tauri::{State, WebviewWindow};

#[derive(Default)]
pub struct AutoHideState {
  suspended_count: AtomicUsize,
}

impl AutoHideState {
  pub fn is_suspended(&self) -> bool {
    self.suspended_count.load(Ordering::SeqCst) > 0
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsCapabilities {
  build: Option<u32>,
  is_undocumented_mica_supported: bool, // >= 22000 (windows 11)
  is_backdroptype_supported: bool,      // >= 22523 (windows 11)
  is_mica_supported: bool,              // backdrop || undocumented_mica
}

#[tauri::command]
pub fn windows_capabilities() -> WindowsCapabilities {
  #[cfg(target_os = "windows")]
  {
    let build = windows_version::OsVersion::current().build;
    let is_undocumented_mica_supported = build >= 22000;
    let is_backdroptype_supported = build >= 22523;
    let is_mica_supported = is_backdroptype_supported || is_undocumented_mica_supported;

    WindowsCapabilities {
      build: Some(build),
      is_undocumented_mica_supported,
      is_backdroptype_supported,
      is_mica_supported,
    }
  }

  #[cfg(not(target_os = "windows"))]
  {
    WindowsCapabilities {
      build: None,
      is_undocumented_mica_supported: false,
      is_backdroptype_supported: false,
      is_mica_supported: false,
    }
  }
}

pub fn is_mica_supported() -> bool {
  windows_capabilities().is_mica_supported
}

#[tauri::command]
pub fn suspend_window_auto_hide(state: State<'_, AutoHideState>) {
  state.suspended_count.fetch_add(1, Ordering::SeqCst);
}

#[tauri::command]
pub fn resume_window_auto_hide(state: State<'_, AutoHideState>) {
  let _ = state
    .suspended_count
    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
      Some(value.saturating_sub(1))
    });
}

#[tauri::command]
pub fn set_window_theme(window: WebviewWindow, theme: String) {
  match theme.as_str() {
    "mica" => {
      if is_mica_supported() {
        #[cfg(target_os = "windows")]
        if let Err(error) = window_vibrancy::apply_mica(&window, Some(true)) {
          log::warn!("[theme] failed to apply mica: {}", error);
        }
      } else {
        log::warn!("[theme] mica is not supported on this system");
      }
    }
    "default" => {
      if is_mica_supported() {
        #[cfg(target_os = "windows")]
        if let Err(error) = window_vibrancy::clear_mica(&window) {
          log::warn!("[theme] failed to clear mica: {}", error);
        }
      }
    }
    _ => {
      log::warn!("[theme] unsupported theme: {}", theme);
    }
  }
}

#[tauri::command]
pub fn get_vscode_recent_from_state(db_path: String) -> Result<String, String> {
  let conn = Connection::open_with_flags(
    db_path,
    OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
  )
  .map_err(|e| e.to_string())?;

  let mut stmt = conn
    .prepare("SELECT value FROM ItemTable WHERE key = 'history.recentlyOpenedPathsList'")
    .map_err(|e| e.to_string())?;

  let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
  if let Some(row) = rows.next().map_err(|e| e.to_string())? {
    let value: String = row.get(0).map_err(|e| e.to_string())?;
    Ok(value)
  } else {
    Err("Key not found".into())
  }
}
