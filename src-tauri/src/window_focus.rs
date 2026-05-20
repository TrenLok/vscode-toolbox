use std::{
  sync::Mutex,
  thread,
  time::{Duration, Instant},
};

use tauri::{AppHandle, Manager, WindowEvent};

use crate::{commands, window_position::set_window_position};

const FOREGROUND_WATCH_START_DELAY_MS: u64 = 1_250;
const FOREGROUND_WATCH_POLL_MS: u64 = 100;
const TRAY_AUTO_HIDE_SUPPRESS_MS: u64 = 350;
const TRAY_REVEAL_BLUR_SUPPRESS_MS: u64 = 600;
const TRAY_REVEAL_BLUR_SUPPRESS_COUNT: u8 = 4;
const WEBVIEW_FOCUS_DELAY_MS: u64 = 250;
const REVEAL_REFOCUS_DELAY_MS: u64 = 80;

#[cfg(windows)]
unsafe extern "system" {
  fn AttachThreadInput(id_attach: u32, id_attach_to: u32, attach: i32) -> i32;
  fn BringWindowToTop(hwnd: *mut std::ffi::c_void) -> i32;
  fn GetCurrentThreadId() -> u32;
  fn GetForegroundWindow() -> *mut std::ffi::c_void;
  fn GetWindowThreadProcessId(hwnd: *mut std::ffi::c_void, process_id: *mut u32) -> u32;
  fn IsChild(parent: *mut std::ffi::c_void, hwnd: *mut std::ffi::c_void) -> i32;
  fn SetActiveWindow(hwnd: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
  fn SetForegroundWindow(hwnd: *mut std::ffi::c_void) -> i32;
  fn SetFocus(hwnd: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
  fn SetWindowPos(
    hwnd: *mut std::ffi::c_void,
    hwnd_insert_after: *mut std::ffi::c_void,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    flags: u32,
  ) -> i32;
  fn ShowWindow(hwnd: *mut std::ffi::c_void, n_cmd_show: i32) -> i32;
}

#[cfg(windows)]
const SW_RESTORE: i32 = 9;
#[cfg(windows)]
const SWP_NOMOVE: u32 = 0x0002;
#[cfg(windows)]
const SWP_NOSIZE: u32 = 0x0001;
#[cfg(windows)]
const SWP_SHOWWINDOW: u32 = 0x0040;
#[cfg(windows)]
const HWND_TOPMOST: *mut std::ffi::c_void = -1isize as *mut std::ffi::c_void;
#[cfg(windows)]
const HWND_NOTOPMOST: *mut std::ffi::c_void = -2isize as *mut std::ffi::c_void;

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

#[derive(Default)]
pub struct WindowToggleState {
  last_auto_hide_at: Mutex<Option<Instant>>,
  last_reveal_at: Mutex<Option<Instant>>,
  ignored_reveal_blur_count: Mutex<u8>,
  did_schedule_reveal_refocus: Mutex<bool>,
}

impl WindowToggleState {
  fn record_reveal(&self) {
    if let Ok(mut last_reveal_at) = self.last_reveal_at.lock() {
      *last_reveal_at = Some(Instant::now());
    }
    if let Ok(mut ignored_reveal_blur_count) = self.ignored_reveal_blur_count.lock() {
      *ignored_reveal_blur_count = 0;
    }
    if let Ok(mut did_schedule_reveal_refocus) = self.did_schedule_reveal_refocus.lock() {
      *did_schedule_reveal_refocus = false;
    }
  }

  fn record_auto_hide(&self) {
    if let Ok(mut last_auto_hide_at) = self.last_auto_hide_at.lock() {
      *last_auto_hide_at = Some(Instant::now());
    }
  }

  fn did_auto_hide_recently(&self) -> bool {
    self
      .last_auto_hide_at
      .lock()
      .ok()
      .and_then(|last_auto_hide_at| *last_auto_hide_at)
      .is_some_and(|instant| instant.elapsed() <= Duration::from_millis(TRAY_AUTO_HIDE_SUPPRESS_MS))
  }

  // Ignores a small number of blur events that can fire immediately after reveal.
  fn should_ignore_reveal_blur(&self) -> bool {
    // Tauri can emit a transient blur while Windows is still assigning focus to the popup.
    let is_initial_reveal_blur = self
      .last_reveal_at
      .lock()
      .ok()
      .and_then(|last_reveal_at| *last_reveal_at)
      .is_some_and(|instant| {
        instant.elapsed() <= Duration::from_millis(TRAY_REVEAL_BLUR_SUPPRESS_MS)
      });

    if !is_initial_reveal_blur {
      return false;
    }

    self
      .ignored_reveal_blur_count
      .lock()
      .is_ok_and(|mut ignored_reveal_blur_count| {
        // Keep the suppression bounded so a real focus loss cannot be ignored forever.
        if *ignored_reveal_blur_count >= TRAY_REVEAL_BLUR_SUPPRESS_COUNT {
          false
        } else {
          *ignored_reveal_blur_count += 1;
          true
        }
      })
  }

  fn should_schedule_reveal_refocus(&self) -> bool {
    self
      .did_schedule_reveal_refocus
      .lock()
      .is_ok_and(|mut did_schedule_reveal_refocus| {
        if *did_schedule_reveal_refocus {
          false
        } else {
          *did_schedule_reveal_refocus = true;
          true
        }
      })
  }
}

#[cfg(windows)]
fn is_foreground_hwnd_or_child(own_hwnd: *mut std::ffi::c_void) -> bool {
  let foreground_hwnd = unsafe { GetForegroundWindow() };

  foreground_hwnd == own_hwnd || unsafe { IsChild(own_hwnd, foreground_hwnd) } != 0
}

#[cfg(windows)]
fn is_foreground_tauri_window_or_child(window: &tauri::Window) -> bool {
  window
    .hwnd()
    .is_ok_and(|hwnd| is_foreground_hwnd_or_child(hwnd.0))
}

#[cfg(not(windows))]
fn is_foreground_tauri_window_or_child(_: &tauri::Window) -> bool {
  true
}

fn request_window_focus(win: &tauri::WebviewWindow) {
  let _ = win.set_focus();

  #[cfg(windows)]
  // Tauri focus alone is not always enough to make the HWND the system foreground window.
  match win.hwnd() {
    Ok(hwnd) => unsafe {
      activate_windows_window(hwnd.0);
    },
    Err(error) => {
      focus_debug_warn!(
        "[focus-debug] request-focus window={} failed-to-get-hwnd error={}",
        win.label(),
        error
      );
    }
  }
}

// Focuses attached webviews now and once again after the window has settled.
fn request_webview_focus(win: tauri::WebviewWindow) {
  request_attached_webviews_focus(&win);

  // The webview may not accept focus immediately after show(), so retry once.
  tauri::async_runtime::spawn_blocking(move || {
    thread::sleep(Duration::from_millis(WEBVIEW_FOCUS_DELAY_MS));
    if win.is_visible().unwrap_or(false) {
      request_attached_webviews_focus(&win);
    }
  });
}

// Restores focus shortly after a reveal when the first blur event looks spurious.
fn schedule_reveal_refocus(win: tauri::WebviewWindow) {
  tauri::async_runtime::spawn_blocking(move || {
    thread::sleep(Duration::from_millis(REVEAL_REFOCUS_DELAY_MS));
    if win.is_visible().unwrap_or(false) {
      request_window_focus(&win);
      request_attached_webviews_focus(&win);
    }
  });
}

fn request_attached_webviews_focus(win: &tauri::WebviewWindow) {
  for (_, webview) in win.webviews() {
    let _ = webview.show();
    let _ = webview.set_focus();
  }
}

#[cfg(windows)]
// Uses WinAPI calls to bring a window to the foreground more reliably on Windows.
unsafe fn activate_windows_window(hwnd: *mut std::ffi::c_void) {
  let foreground_hwnd = GetForegroundWindow();
  let foreground_thread_id = if foreground_hwnd.is_null() {
    0
  } else {
    GetWindowThreadProcessId(foreground_hwnd, std::ptr::null_mut())
  };
  let current_thread_id = GetCurrentThreadId();
  // Windows restricts foreground changes across input queues; attach briefly if needed.
  let attached = foreground_thread_id != 0
    && foreground_thread_id != current_thread_id
    && AttachThreadInput(current_thread_id, foreground_thread_id, 1) != 0;

  let _ = ShowWindow(hwnd, SW_RESTORE);
  // Bounce through topmost to raise the window without leaving it permanently always-on-top.
  let _ = SetWindowPos(
    hwnd,
    HWND_TOPMOST,
    0,
    0,
    0,
    0,
    SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
  );
  let _ = SetWindowPos(
    hwnd,
    HWND_NOTOPMOST,
    0,
    0,
    0,
    0,
    SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
  );
  let _ = BringWindowToTop(hwnd);
  // These calls cover different parts of Windows focus/activation state.
  let _ = SetActiveWindow(hwnd);
  let _ = SetFocus(hwnd);
  let _ = SetForegroundWindow(hwnd);

  if attached {
    let _ = AttachThreadInput(current_thread_id, foreground_thread_id, 0);
  }
}

#[cfg(windows)]
// Watches the real Windows foreground window and hides this window when focus leaves.
fn start_foreground_watcher(win: tauri::WebviewWindow) {
  let window_label = win.label().to_string();
  tauri::async_runtime::spawn_blocking(move || {
    // Let the initial reveal/focus sequence settle before treating foreground mismatch as blur.
    thread::sleep(Duration::from_millis(FOREGROUND_WATCH_START_DELAY_MS));

    loop {
      let is_visible = win.is_visible().unwrap_or(false);
      if !is_visible {
        break;
      }

      let own_hwnd = match win.hwnd() {
        Ok(hwnd) => hwnd.0,
        Err(error) => {
          focus_debug_warn!(
            "[focus-debug] foreground-watch window={} failed-to-get-hwnd error={}",
            window_label,
            error
          );
          break;
        }
      };

      if !is_foreground_hwnd_or_child(own_hwnd) {
        let foreground_hwnd = unsafe { GetForegroundWindow() };
        let toggle_state = win.app_handle().state::<WindowToggleState>();

        // If Tauri still reports focus, wait for the native and Tauri states to converge.
        if win.is_focused().unwrap_or(false) {
          focus_debug_info!(
            "[focus-debug] foreground-mismatch-skipped window={} reason=tauri-focused own_hwnd={:#x} foreground_hwnd={:#x}",
            window_label,
            own_hwnd as isize,
            foreground_hwnd as isize
          );
          thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
          continue;
        }

        if toggle_state.should_ignore_reveal_blur() {
          focus_debug_info!(
            "[focus-debug] foreground-mismatch-skipped window={} reason=initial-reveal-blur own_hwnd={:#x} foreground_hwnd={:#x}",
            window_label,
            own_hwnd as isize,
            foreground_hwnd as isize
          );
          thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
          continue;
        }

        focus_debug_info!(
          "[focus-debug] foreground-mismatch window={} own_hwnd={:#x} foreground_hwnd={:#x}",
          window_label,
          own_hwnd as isize,
          foreground_hwnd as isize
        );
        toggle_state.record_auto_hide();
        let _ = win.hide();
        break;
      }

      thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
    }
  });
}

#[cfg(not(windows))]
fn start_foreground_watcher(_: tauri::WebviewWindow) {}

pub fn toggle_window(app: &AppHandle, label: &str, tray_rect: Option<tauri::Rect>) {
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
    let minimized = win.is_minimized().unwrap_or(false);
    if visible && !minimized {
      focus_debug_info!("[focus-debug] toggle-hide window={}", label);
      let _ = win.hide();
    } else {
      reveal_window(app, label, tray_rect);
    }
  }
}

pub fn reveal_window(app: &AppHandle, label: &str, tray_rect: Option<tauri::Rect>) {
  let auto_hide_state = app.state::<commands::AutoHideState>();
  if auto_hide_state.is_suspended() {
    focus_debug_info!(
      "[focus-debug] reveal-skipped window={} reason=suspended",
      label
    );
    return;
  }

  if let Some(win) = app.get_webview_window(label) {
    let minimized = win.is_minimized().unwrap_or(false);
    let toggle_state = app.state::<WindowToggleState>();
    // Tray clicks can arrive as part of the same interaction that caused auto-hide.
    if tray_rect.is_some() && toggle_state.did_auto_hide_recently() {
      focus_debug_info!(
        "[focus-debug] reveal-skipped window={} reason=recent-auto-hide",
        label
      );
      return;
    }

    toggle_state.record_reveal();

    let _ = set_window_position(&win, tray_rect);
    let _ = win.show();
    if minimized {
      let _ = win.unminimize();
    }
    request_window_focus(&win);
    request_webview_focus(win.clone());
    focus_debug_info!("[focus-debug] reveal window={}", label);
    start_foreground_watcher(win);
  }
}

pub fn on_window_event(window: &tauri::Window, event: &WindowEvent) {
  if let WindowEvent::Focused(is_focused) = event {
    focus_debug_info!(
      "[focus-debug] focused-event window={} focused={}",
      window.label(),
      is_focused
    );

    if *is_focused {
      return;
    }

    let auto_hide_state = window.app_handle().state::<commands::AutoHideState>();
    if auto_hide_state.is_suspended() {
      focus_debug_info!(
        "[focus-debug] auto-hide-skipped window={} reason=suspended",
        window.label()
      );
      return;
    }

    let toggle_state = window.app_handle().state::<WindowToggleState>();
    // A blur right after reveal can be synthetic; refocus once instead of hiding.
    if is_foreground_tauri_window_or_child(window) && toggle_state.should_ignore_reveal_blur() {
      focus_debug_info!(
        "[focus-debug] auto-hide-skipped window={} reason=initial-reveal-blur",
        window.label()
      );
      if toggle_state.should_schedule_reveal_refocus() {
        if let Some(win) = window.app_handle().get_webview_window(window.label()) {
          schedule_reveal_refocus(win);
        }
      }
      return;
    }

    focus_debug_info!("[focus-debug] auto-hide window={}", window.label());
    toggle_state.record_auto_hide();
    let _ = window.hide();
  }
}
