#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod macos_vscode;
#[cfg(target_os = "macos")]
mod macos_window;
mod reveal_shortcut;
mod tray;
mod window_focus;
mod window_position;
mod window_theme;
mod windows_vscode;

use tauri::{
  menu::{Menu, MenuItem},
  Manager,
};

pub fn run() {
  let builder = tauri::Builder::default()
    .manage(commands::AutoHideState::default())
    .manage(window_focus::WindowToggleState::default())
    .manage(reveal_shortcut::RevealShortcutState::default())
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

  #[cfg(not(any(target_os = "android", target_os = "ios")))]
  let builder = builder.plugin(
    tauri_plugin_global_shortcut::Builder::new()
      .with_handler(|app, shortcut, event| {
        reveal_shortcut::handle_shortcut(app, shortcut, event);
      })
      .build(),
  );

  #[cfg(not(debug_assertions))]
  let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
    if app.get_webview_window("main").is_some() {
      window_focus::toggle_window(app, "main", tray::rect(app));
    } else {
      log::warn!("[single-instance] main window is not available");
    }
  }));

  builder
    .on_window_event(|window, event| {
      window_focus::on_window_event(window, event);
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
      window_theme::apply_from_settings(app, &win)?;

      #[cfg(target_os = "macos")]
      if let Err(error) = macos_window::apply_window_corner_radius(&win) {
        log::warn!(
          "[startup] failed to apply macOS window corner radius: {}",
          error
        );
      }

      #[cfg(debug_assertions)]
      win.open_devtools();

      if let Err(error) = tray::setup(app, &menu) {
        log::error!("[startup] failed to setup tray icon: {}", error);
      }

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::available_themes,
      commands::get_vscode_recent_from_state,
      commands::has_vscode_recent_state_key,
      commands::suspend_window_auto_hide,
      commands::resume_window_auto_hide,
      macos_vscode::get_vscode_version_macos,
      macos_vscode::open_vscode_project_macos,
      macos_vscode::open_vscode_project_uri_macos,
      windows_vscode::get_vscode_version_windows,
      windows_vscode::open_vscode_project_windows,
      windows_vscode::open_vscode_project_uri_windows,
      commands::windows_capabilities,
      commands::set_window_theme,
      reveal_shortcut::set_reveal_shortcut,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
