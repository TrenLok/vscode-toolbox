use std::sync::atomic::{AtomicUsize, Ordering};

use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use tauri::{State, WebviewWindow};

const DEFAULT_THEME: &str = "default";
const MICA_THEME: &str = "mica";
const LIQUID_GLASS_THEME: &str = "liquid_glass";
const VIBRANCY_THEME: &str = "vibrancy";

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

pub fn is_liquid_glass_supported() -> bool {
  #[cfg(target_os = "macos")]
  {
    unsafe { objc2_app_kit::NSAppKitVersionNumber >= 2685.0 }
  }

  #[cfg(not(target_os = "macos"))]
  {
    false
  }
}

pub fn is_vibrancy_supported() -> bool {
  #[cfg(target_os = "macos")]
  {
    unsafe { objc2_app_kit::NSAppKitVersionNumber >= objc2_app_kit::NSAppKitVersionNumber10_10 }
  }

  #[cfg(not(target_os = "macos"))]
  {
    false
  }
}

#[tauri::command]
pub fn available_themes() -> Vec<String> {
  let mut themes = vec![DEFAULT_THEME.to_string()];

  if is_mica_supported() {
    themes.push(MICA_THEME.to_string());
  }

  if is_liquid_glass_supported() {
    themes.push(LIQUID_GLASS_THEME.to_string());
  }

  if is_vibrancy_supported() {
    themes.push(VIBRANCY_THEME.to_string());
  }

  themes
}

pub fn normalize_theme(theme: &str) -> &'static str {
  match theme {
    DEFAULT_THEME => DEFAULT_THEME,
    MICA_THEME if is_mica_supported() => MICA_THEME,
    LIQUID_GLASS_THEME if is_liquid_glass_supported() => LIQUID_GLASS_THEME,
    VIBRANCY_THEME if is_vibrancy_supported() => VIBRANCY_THEME,
    _ => DEFAULT_THEME,
  }
}

fn clear_window_theme(window: &WebviewWindow) {
  #[cfg(target_os = "windows")]
  if let Err(error) = window_vibrancy::clear_mica(window) {
    log::warn!("[theme] failed to clear mica: {}", error);
  }

  #[cfg(target_os = "macos")]
  {
    if let Err(error) = window_vibrancy::clear_liquid_glass(window) {
      log::warn!("[theme] failed to clear liquid glass: {}", error);
    }
    if let Err(error) = window_vibrancy::clear_vibrancy(window) {
      log::warn!("[theme] failed to clear liquid glass: {}", error);
    }
  }
}

pub fn apply_window_theme(window: &WebviewWindow, theme: &str) {
  let theme = normalize_theme(theme);

  clear_window_theme(window);

  match theme {
    DEFAULT_THEME => {}
    MICA_THEME =>
    {
      #[cfg(target_os = "windows")]
      if let Err(error) = window_vibrancy::apply_mica(window, Some(true)) {
        log::warn!("[theme] failed to apply mica: {}", error);
      }
    }
    LIQUID_GLASS_THEME => {
      #[cfg(target_os = "macos")]
      if let Err(error) = window_vibrancy::apply_liquid_glass(
        window,
        window_vibrancy::NSGlassEffectViewStyle::Regular,
        None,
        Some(8.0),
      ) {
        log::warn!("[theme] failed to apply liquid glass: {}", error);
      }
    }
    VIBRANCY_THEME => {
      #[cfg(target_os = "macos")]
      if let Err(error) = window_vibrancy::apply_vibrancy(
        window,
        window_vibrancy::NSVisualEffectMaterial::HudWindow,
        None,
        Some(8.0),
      ) {
        log::warn!("[theme] failed to apply vibrancy: {}", error);
      }
    }
    _ => unreachable!(),
  }
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
  let normalized_theme = normalize_theme(&theme);
  if normalized_theme != theme {
    log::warn!(
      "[theme] unsupported or unavailable theme requested: {}, falling back to {}",
      theme,
      normalized_theme
    );
  }

  let window_for_main_thread = window.clone();
  if let Err(error) = window.run_on_main_thread(move || {
    apply_window_theme(&window_for_main_thread, normalized_theme);
  }) {
    log::warn!(
      "[theme] failed to schedule theme apply on main thread: {}",
      error
    );
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
