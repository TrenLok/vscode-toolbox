use std::{
  sync::Mutex,
  thread,
  time::{Duration, Instant},
};

use tauri::{AppHandle, Manager, WindowEvent};

#[cfg(windows)]
use crate::windows_focus;
use crate::{commands, window_position::set_window_position};

// Waits for Windows to finish the initial foreground/focus negotiation after reveal.
const FOREGROUND_WATCH_START_DELAY_MS: u64 = 1_250;
const FOREGROUND_WATCH_POLL_MS: u64 = 100;
const TRAY_AUTO_HIDE_SUPPRESS_MS: u64 = 350;
// Tauri/Windows can emit several synthetic blur events immediately after show/focus.
const TRAY_REVEAL_BLUR_SUPPRESS_MS: u64 = 600;
const TRAY_REVEAL_BLUR_SUPPRESS_COUNT: u8 = 4;
// WebView2 sometimes rejects focus during the first window-show frame.
const WEBVIEW_FOCUS_DELAY_MS: u64 = 250;
// A short refocus is enough to recover from the first synthetic reveal blur.
const REVEAL_REFOCUS_DELAY_MS: u64 = 80;

// Debug logging

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

// State

#[derive(Default)]
pub struct WindowFocusState {
  inner: Mutex<WindowFocusStateInner>,
}

#[derive(Default)]
struct WindowFocusStateInner {
  last_auto_hide_at: Option<Instant>,
  last_reveal_at: Option<Instant>,
  ignored_reveal_blur_count: u8,
  did_schedule_reveal_refocus: bool,
  // Invalidates foreground watchers from previous reveal attempts.
  foreground_watcher_generation: u64,
}

impl WindowFocusState {
  fn record_reveal(&self) {
    if let Ok(mut inner) = self.inner.lock() {
      inner.last_reveal_at = Some(Instant::now());
      inner.ignored_reveal_blur_count = 0;
      inner.did_schedule_reveal_refocus = false;
      inner.foreground_watcher_generation = inner.foreground_watcher_generation.wrapping_add(1);
    }
  }

  fn record_auto_hide(&self) {
    if let Ok(mut inner) = self.inner.lock() {
      inner.last_auto_hide_at = Some(Instant::now());
    }
  }

  fn did_auto_hide_recently(&self) -> bool {
    self
      .inner
      .lock()
      .ok()
      .and_then(|inner| inner.last_auto_hide_at)
      .is_some_and(|instant| instant.elapsed() <= Duration::from_millis(TRAY_AUTO_HIDE_SUPPRESS_MS))
  }

  // Ignores a small number of blur events that can fire immediately after reveal.
  fn should_ignore_reveal_blur(&self) -> bool {
    self.inner.lock().is_ok_and(|mut inner| {
      // Tauri can emit a transient blur while Windows is still assigning focus to the popup.
      let is_initial_reveal_blur = inner.last_reveal_at.is_some_and(|instant| {
        instant.elapsed() <= Duration::from_millis(TRAY_REVEAL_BLUR_SUPPRESS_MS)
      });

      if !is_initial_reveal_blur {
        return false;
      }

      // Keep the suppression bounded so a real focus loss cannot be ignored forever.
      if inner.ignored_reveal_blur_count >= TRAY_REVEAL_BLUR_SUPPRESS_COUNT {
        false
      } else {
        inner.ignored_reveal_blur_count += 1;
        true
      }
    })
  }

  fn should_schedule_reveal_refocus(&self) -> bool {
    self.inner.lock().is_ok_and(|mut inner| {
      if inner.did_schedule_reveal_refocus {
        false
      } else {
        inner.did_schedule_reveal_refocus = true;
        true
      }
    })
  }

  fn foreground_watcher_generation(&self) -> u64 {
    self
      .inner
      .lock()
      .map(|inner| inner.foreground_watcher_generation)
      .unwrap_or_default()
  }

  fn is_foreground_watcher_current(&self, generation: u64) -> bool {
    self
      .inner
      .lock()
      .is_ok_and(|inner| inner.foreground_watcher_generation == generation)
  }
}

// Foreground checks

#[cfg(windows)]
fn is_foreground_hwnd_or_child(own_hwnd: *mut std::ffi::c_void) -> bool {
  windows_focus::is_foreground_hwnd_or_child(own_hwnd)
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

// Focus requests

fn request_window_focus(win: &tauri::WebviewWindow) {
  let _ = win.set_focus();

  #[cfg(windows)]
  // Tauri focus alone is not always enough to make the HWND the system foreground window.
  match win.hwnd() {
    Ok(hwnd) => unsafe {
      windows_focus::activate_window(hwnd.0);
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

fn is_window_visible(win: &tauri::WebviewWindow) -> bool {
  win.is_visible().unwrap_or(false)
}

fn is_window_focused(win: &tauri::WebviewWindow) -> bool {
  win.is_focused().unwrap_or(false)
}

fn wait_foreground_poll_interval() {
  thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
}

fn schedule_visible_window_task<F>(win: tauri::WebviewWindow, delay_ms: u64, task: F)
where
  F: FnOnce(&tauri::WebviewWindow) + Send + 'static,
{
  tauri::async_runtime::spawn_blocking(move || {
    thread::sleep(Duration::from_millis(delay_ms));
    if is_window_visible(&win) {
      task(&win);
    }
  });
}

// Focuses attached webviews now and once again after the window has settled.
fn request_webview_focus(win: tauri::WebviewWindow) {
  request_attached_webviews_focus(&win);

  // The webview may not accept focus immediately after show(), so retry once.
  schedule_visible_window_task(win, WEBVIEW_FOCUS_DELAY_MS, request_attached_webviews_focus);
}

// Restores focus shortly after a reveal when the first blur event looks spurious.
fn schedule_reveal_refocus(win: tauri::WebviewWindow) {
  schedule_visible_window_task(win, REVEAL_REFOCUS_DELAY_MS, |win| {
    request_window_focus(win);
    request_attached_webviews_focus(win);
  });
}

fn request_attached_webviews_focus(win: &tauri::WebviewWindow) {
  for (_, webview) in win.webviews() {
    let _ = webview.set_focus();
  }
}

// Foreground watcher

#[cfg(windows)]
fn get_window_hwnd(
  win: &tauri::WebviewWindow,
  window_label: &str,
) -> Option<*mut std::ffi::c_void> {
  match win.hwnd() {
    Ok(hwnd) => Some(hwnd.0),
    Err(error) => {
      focus_debug_warn!(
        "[focus-debug] foreground-watch window={} failed-to-get-hwnd error={}",
        window_label,
        error
      );
      None
    }
  }
}

#[cfg(windows)]
// Watches the real Windows foreground window and hides this window when focus leaves.
fn start_foreground_watcher(win: tauri::WebviewWindow) {
  let window_label = win.label().to_string();
  let generation = win
    .app_handle()
    .state::<WindowFocusState>()
    .foreground_watcher_generation();

  tauri::async_runtime::spawn_blocking(move || {
    // Let the initial reveal/focus sequence settle before treating foreground mismatch as blur.
    thread::sleep(Duration::from_millis(FOREGROUND_WATCH_START_DELAY_MS));

    loop {
      let focus_state = win.app_handle().state::<WindowFocusState>();
      if !focus_state.is_foreground_watcher_current(generation) {
        focus_debug_info!(
          "[focus-debug] foreground-watch-stopped window={} reason=stale-generation",
          window_label
        );
        break;
      }

      if !is_window_visible(&win) {
        break;
      }

      let Some(own_hwnd) = get_window_hwnd(&win, &window_label) else {
        break;
      };

      if !is_foreground_hwnd_or_child(own_hwnd) {
        if should_defer_foreground_loss(&win, &focus_state, own_hwnd, &window_label) {
          continue;
        }

        hide_after_foreground_loss(&win, &focus_state, own_hwnd, &window_label);
        break;
      }

      thread::sleep(Duration::from_millis(FOREGROUND_WATCH_POLL_MS));
    }
  });
}

#[cfg(windows)]
fn should_defer_foreground_loss(
  win: &tauri::WebviewWindow,
  focus_state: &WindowFocusState,
  own_hwnd: *mut std::ffi::c_void,
  window_label: &str,
) -> bool {
  let foreground_hwnd = windows_focus::foreground_hwnd();

  // If Tauri still reports focus, wait for the native and Tauri states to converge.
  if is_window_focused(win) {
    focus_debug_info!(
      "[focus-debug] foreground-mismatch-skipped window={} reason=tauri-focused own_hwnd={:#x} foreground_hwnd={:#x}",
      window_label,
      own_hwnd as isize,
      foreground_hwnd as isize
    );
    wait_foreground_poll_interval();
    return true;
  }

  if focus_state.should_ignore_reveal_blur() {
    focus_debug_info!(
      "[focus-debug] foreground-mismatch-skipped window={} reason=initial-reveal-blur own_hwnd={:#x} foreground_hwnd={:#x}",
      window_label,
      own_hwnd as isize,
      foreground_hwnd as isize
    );
    wait_foreground_poll_interval();
    return true;
  }

  false
}

#[cfg(windows)]
fn hide_after_foreground_loss(
  win: &tauri::WebviewWindow,
  focus_state: &WindowFocusState,
  own_hwnd: *mut std::ffi::c_void,
  window_label: &str,
) {
  let foreground_hwnd = windows_focus::foreground_hwnd();

  focus_debug_info!(
    "[focus-debug] foreground-mismatch window={} own_hwnd={:#x} foreground_hwnd={:#x}",
    window_label,
    own_hwnd as isize,
    foreground_hwnd as isize
  );
  focus_state.record_auto_hide();
  let _ = win.hide();
}

#[cfg(not(windows))]
fn start_foreground_watcher(_: tauri::WebviewWindow) {}

// Public API

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
    let focus_state = app.state::<WindowFocusState>();
    // Tray clicks can arrive as part of the same interaction that caused auto-hide.
    if tray_rect.is_some() && focus_state.did_auto_hide_recently() {
      focus_debug_info!(
        "[focus-debug] reveal-skipped window={} reason=recent-auto-hide",
        label
      );
      return;
    }

    focus_state.record_reveal();

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

    let focus_state = window.app_handle().state::<WindowFocusState>();
    // A blur right after reveal can be synthetic; refocus once instead of hiding.
    if is_foreground_tauri_window_or_child(window) && focus_state.should_ignore_reveal_blur() {
      focus_debug_info!(
        "[focus-debug] auto-hide-skipped window={} reason=initial-reveal-blur",
        window.label()
      );
      if focus_state.should_schedule_reveal_refocus() {
        if let Some(win) = window.app_handle().get_webview_window(window.label()) {
          schedule_reveal_refocus(win);
        }
      }
      return;
    }

    focus_debug_info!("[focus-debug] auto-hide window={}", window.label());
    focus_state.record_auto_hide();
    let _ = window.hide();
  }
}
