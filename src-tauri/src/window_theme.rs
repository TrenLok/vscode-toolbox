use tauri::{App, Wry};
use tauri_plugin_store::{JsonValue, StoreExt};

use crate::commands;

pub fn apply_from_settings(
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
