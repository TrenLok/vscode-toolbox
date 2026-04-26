#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod macos_vscode;
#[cfg(target_os = "macos")]
mod macos_window;
mod window_position;

use std::{thread, time::Duration};

use crate::window_position::set_window_position;
use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Manager, WindowEvent, Wry,
};
use tauri_plugin_store::{JsonValue, StoreExt};

const FOCUS_RETRY_COUNT: usize = 6;
const FOCUS_RETRY_DELAY_MS: u64 = 50;
const FOREGROUND_WATCH_START_DELAY_MS: u64 = 250;
const FOREGROUND_WATCH_POLL_MS: u64 = 100;

#[cfg(windows)]
unsafe extern "system" {
  fn GetForegroundWindow() -> *mut std::ffi::c_void;
}

#[cfg(debug_assertions)]
macro_rules! focus_debug_info {
  ($($arg:tt)*) => {
    log::info!($($arg)*);
  };
}

#[cfg(not(debug_assertions))]
macro_rules! focus_debug_info {
  ($($arg:tt)*) => {{
    let _ = format_args!($($arg)*);
  }};
}

#[cfg(debug_assertions)]
macro_rules! focus_debug_warn {
  ($($arg:tt)*) => {
    log::warn!($($arg)*);
  };
}

#[cfg(not(debug_assertions))]
macro_rules! focus_debug_warn {
  ($($arg:tt)*) => {{
    let _ = format_args!($($arg)*);
  }};
}

fn ensure_window_focus(win: tauri::WebviewWindow) {
  let window_label = win.label().to_string();
  tauri::async_runtime::spawn_blocking(move || {
    for attempt in 1..=FOCUS_RETRY_COUNT {
      let is_focused = win.is_focused().unwrap_or(false);
      focus_debug_info!(
        "[focus-debug] focus-retry window={} attempt={}/{} focused={}",
        window_label,
        attempt,
        FOCUS_RETRY_COUNT,
        is_focused
      );

      if is_focused {
        break;
      }

      let _ = win.set_focus();
      thread::sleep(Duration::from_millis(FOCUS_RETRY_DELAY_MS));
    }
  });
}

#[cfg(windows)]
fn start_foreground_watcher(win: tauri::WebviewWindow) {
  let window_label = win.label().to_string();
  tauri::async_runtime::spawn_blocking(move || {
    thread::sleep(Duration::from_millis(FOREGROUND_WATCH_START_DELAY_MS));

    loop {
      let is_visible = win.is_visible().unwrap_or(false);
      if !is_visible {
        break;
      }

      let own_hwnd = match win.hwnd() {
        Ok(hwnd) => hwnd.0 as isize,
        Err(error) => {
          focus_debug_warn!(
            "[focus-debug] foreground-watch window={} failed-to-get-hwnd error={}",
            window_label,
            error
          );
          break;
        }
      };

      let foreground_hwnd = unsafe { GetForegroundWindow() } as isize;

      if foreground_hwnd != own_hwnd {
        focus_debug_info!(
          "[focus-debug] foreground-mismatch window={} own_hwnd={:#x} foreground_hwnd={:#x}",
          window_label,
          own_hwnd,
          foreground_hwnd
        );
        let _ = win.hide();
        break;
      }

      thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
    }
  });
}

#[cfg(not(windows))]
fn start_foreground_watcher(_: tauri::WebviewWindow) {}

fn toggle_window(app: &AppHandle, label: &str, tray_rect: Option<tauri::Rect>) {
  let auto_hide_state = app.state::<commands::AutoHideState>();
  if auto_hide_state.is_suspended() {
    focus_debug_info!(
      "[focus-debug] toggle-skipped window={} reason=suspended",
      label
    );
    return;
  }

  if let Some(win) = app.get_webview_window(label) {
    let visible = win.is_visible().unwrap_or(false);
    if visible {
      focus_debug_info!("[focus-debug] toggle-hide window={}", label);
      let _ = win.hide();
    } else {
      let _ = set_window_position(&win, tray_rect);
      let _ = win.show();
      let _ = win.set_focus();
      focus_debug_info!("[focus-debug] toggle-show window={}", label);
      ensure_window_focus(win.clone());
      start_foreground_watcher(win);
    }
  }
}

fn apply_window_theme_from_settings(
  app: &App<Wry>,
  win: &tauri::WebviewWindow,
) -> Result<(), Box<dyn std::error::Error>> {
  let settings_store = app.store("settings.json")?;
  if let Some(mut settings) = settings_store.get("settings") {
    if let Some(theme) = settings.pointer("/data/theme").and_then(JsonValue::as_str) {
      let normalized_theme = commands::normalize_theme(theme);

      if normalized_theme != theme {
        if let Some(theme_value) = settings.pointer_mut("/data/theme") {
          *theme_value = JsonValue::String(normalized_theme.to_string());
        }
        settings_store.set("settings", settings);
        settings_store.save()?;
      }

      commands::apply_window_theme(win, normalized_theme);
    }
  }

  Ok(())
}

#[allow(unused_variables)]
fn setup_tray_icon(app: &mut App<Wry>, menu: &Menu<Wry>) -> Result<(), Box<dyn std::error::Error>> {
  let mut tray_builder = TrayIconBuilder::new()
    .menu(menu)
    .show_menu_on_left_click(false);

  if let Some(icon) = app.default_window_icon().cloned() {
    tray_builder = tray_builder.icon(icon);
  }

  let tray = tray_builder
    .on_tray_icon_event(|tray, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        rect,
        ..
      } = event
      {
        let app = tray.app_handle();
        toggle_window(app, "main", Some(rect))
      }
    })
    .on_menu_event(|app, event| {
      if event.id.as_ref() == "quit" {
        app.exit(0);
      }
    })
    .build(app)?;

  Ok(())
}

pub fn run() {
  let builder = tauri::Builder::default()
    .manage(commands::AutoHideState::default())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_autostart::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(
      tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .max_file_size(50_000 /* bytes */)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        .format(|out, message, record| {
          let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
          out.finish(format_args!(
            "[{}][{}][{}] {}",
            date,
            record.level(),
            record.target(),
            message
          ))
        })
        .build(),
    );

  #[cfg(not(debug_assertions))]
  let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
    if app.get_webview_window("main").is_some() {
      toggle_window(app, "main", None);
    } else {
      log::warn!("[single-instance] main window is not available");
    }
  }));

  builder
    .on_window_event(|window, event| {
      if let WindowEvent::Focused(is_focused) = event {
        focus_debug_info!(
          "[focus-debug] focused-event window={} focused={}",
          window.label(),
          is_focused
        );

        if !is_focused {
          let auto_hide_state = window.app_handle().state::<commands::AutoHideState>();
          if auto_hide_state.is_suspended() {
            focus_debug_info!(
              "[focus-debug] auto-hide-skipped window={} reason=suspended",
              window.label()
            );
            return;
          }

          focus_debug_info!("[focus-debug] auto-hide window={}", window.label());
          let _ = window.hide();
        }
      }
    })
    .setup(|app| {
      #[cfg(target_os = "macos")]
      {
        app.set_dock_visibility(false);
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
      }

      let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

      let menu = Menu::with_items(app, &[&quit_i])?;
      let win = app.get_webview_window("main").unwrap();
      apply_window_theme_from_settings(app, &win)?;

      #[cfg(target_os = "macos")]
      if let Err(error) = macos_window::apply_window_corner_radius(&win) {
        log::warn!(
          "[startup] failed to apply macOS window corner radius: {}",
          error
        );
      }

      #[cfg(debug_assertions)]
      win.open_devtools();

      set_window_position(&win, None)?;

      let tray_setup_result = setup_tray_icon(app, &menu);
      if let Err(error) = &tray_setup_result {
        log::error!("[startup] failed to setup tray icon: {}", error);
      }

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::available_themes,
      commands::get_vscode_recent_from_state,
      commands::suspend_window_auto_hide,
      commands::resume_window_auto_hide,
      macos_vscode::get_vscode_version_macos,
      macos_vscode::open_vscode_project_macos,
      commands::windows_capabilities,
      commands::set_window_theme,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
