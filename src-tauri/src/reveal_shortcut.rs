use std::{str::FromStr, sync::Mutex};

use tauri::{AppHandle, Manager, State};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::{tray, window_focus};

#[derive(Default)]
pub struct RevealShortcutState {
  current: Mutex<Option<Shortcut>>,
}

impl RevealShortcutState {
  fn current(&self) -> Option<Shortcut> {
    self.current.lock().ok().and_then(|current| *current)
  }

  fn set_current(&self, shortcut: Shortcut) {
    if let Ok(mut current) = self.current.lock() {
      *current = Some(shortcut);
    }
  }

  fn clear_current(&self) {
    if let Ok(mut current) = self.current.lock() {
      *current = None;
    }
  }
}

pub fn handle_shortcut(
  app: &AppHandle,
  shortcut: &Shortcut,
  event: tauri_plugin_global_shortcut::ShortcutEvent,
) {
  if event.state() != ShortcutState::Pressed {
    return;
  }

  let state = app.state::<RevealShortcutState>();
  if state.current().is_some_and(|current| current == *shortcut) {
    window_focus::reveal_window(app, "main", tray::rect(app));
  }
}

fn register_with_state(
  app: &AppHandle,
  state: &State<'_, RevealShortcutState>,
  shortcut: Option<String>,
) -> Result<(), String> {
  let Some(shortcut) = shortcut.filter(|value| !value.trim().is_empty()) else {
    if let Some(current) = state.current() {
      app
        .global_shortcut()
        .unregister(current)
        .map_err(|error| error.to_string())?;
      state.clear_current();
    }
    return Ok(());
  };

  let next = Shortcut::from_str(&shortcut).map_err(|error| error.to_string())?;

  if state.current().is_some_and(|current| current == next) {
    return Ok(());
  }

  if let Some(current) = state.current() {
    app
      .global_shortcut()
      .unregister(current)
      .map_err(|error| error.to_string())?;
  }

  match app.global_shortcut().register(next) {
    Ok(()) => {
      state.set_current(next);
      Ok(())
    }
    Err(error) => {
      if let Some(current) = state.current() {
        let _ = app.global_shortcut().register(current);
      }
      Err(error.to_string())
    }
  }
}

#[tauri::command]
pub fn set_reveal_shortcut(
  app: AppHandle,
  state: State<'_, RevealShortcutState>,
  shortcut: Option<String>,
) -> Result<(), String> {
  register_with_state(&app, &state, shortcut)
}
