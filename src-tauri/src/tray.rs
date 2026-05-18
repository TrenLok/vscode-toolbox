use tauri::{
  menu::Menu,
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, Wry,
};

use crate::window_focus;

pub fn setup(app: &mut App<Wry>, menu: &Menu<Wry>) -> Result<(), Box<dyn std::error::Error>> {
  let mut tray_builder = TrayIconBuilder::new()
    .menu(menu)
    .icon_as_template(cfg!(target_os = "macos"))
    .tooltip("VSCode Toolbox")
    .show_menu_on_left_click(false);

  #[cfg(target_os = "macos")]
  {
    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray-macos.png"))?;
    tray_builder = tray_builder.icon(icon);
  }

  #[cfg(not(target_os = "macos"))]
  if let Some(icon) = app.default_window_icon().cloned() {
    tray_builder = tray_builder.icon(icon);
  }

  tray_builder
    .on_tray_icon_event(|tray, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Down,
        rect,
        ..
      } = event
      {
        let app = tray.app_handle();
        window_focus::toggle_window(app, "main", Some(rect))
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
