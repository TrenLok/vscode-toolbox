#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;

use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, WindowEvent, Wry,
};

fn toggle_window(app: &AppHandle, label: &str) {
  if let Some(win) = app.get_webview_window(label) {
    let visible = win.is_visible().unwrap_or(false);
    if visible {
      let _ = win.hide();
    } else {
      let _ = set_window_position(&win);
      let _ = win.set_always_on_top(true);
      let _ = win.show();
      let _ = win.set_focus();
    }
  }
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
        ..
      } = event
      {
        let app = tray.app_handle();
        toggle_window(app, "main")
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
  tauri::Builder::default()
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
    )
    .on_window_event(|window, event| {
      if let WindowEvent::Focused(is_focused) = event {
        if !is_focused {
          let _ = window.hide();
        }
      }
    })
    .setup(|app| {
      let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

      let menu = Menu::with_items(app, &[&quit_i])?;
      let win = app.get_webview_window("main").unwrap();

      #[cfg(debug_assertions)]
      win.open_devtools();

      set_window_position(&win)?;

      let _ = setup_tray_icon(app, &menu);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_vscode_recent_from_state
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn set_window_position(win: &tauri::WebviewWindow) -> tauri::Result<()> {
  let Some(monitor) = win.current_monitor()? else {
    return Ok(());
  };
  let monitor_work_area = monitor.work_area(); // уже без панели задач/дока

  // Window size based on shadow
  let window_size_outer = PhysicalSize::<i32> {
    width: win.outer_size()?.width as i32,
    height: win.outer_size()?.height as i32,
  };

  // Window size without shadow
  let window_size_inner = PhysicalSize::<i32> {
    width: win.inner_size()?.width as i32,
    height: win.inner_size()?.height as i32,
  };

  let work_area_size = PhysicalSize::<i32> {
    width: monitor_work_area.size.width as i32,
    height: monitor_work_area.size.height as i32,
  };

  let work_area_position = PhysicalPosition::<i32> {
    x: monitor_work_area.position.x,
    y: monitor_work_area.position.y,
  };

  let padding = 1;

  let shadow_left_right = (window_size_outer.width - window_size_inner.width) / 2;
  let shadow_bottom = shadow_left_right;

  let x = if work_area_position.x > 0 {
    work_area_position.x - shadow_left_right
  } else {
    work_area_position.x + work_area_size.width - window_size_outer.width + shadow_left_right
      - padding
  };

  let y = if work_area_position.y > 0 {
    work_area_position.y
  } else {
    work_area_position.y + work_area_size.height - window_size_outer.height + shadow_bottom
      - padding
  };

  win.set_position(Position::Physical(PhysicalPosition { x, y }))?;
  Ok(())
}
