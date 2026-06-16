use objc2_app_kit::{NSColor, NSView, NSWindow};
use objc2_quartz_core::CALayer;
use tauri::Manager;
use tauri_nspanel::objc2::{ClassType, Message};
use tauri_nspanel::objc2_foundation::NSObjectProtocol;
use tauri_nspanel::{panel, CollectionBehavior, PanelLevel, StyleMask, WebviewWindowExt};

const MACOS_WINDOW_CORNER_RADIUS: f64 = 8.0;

panel!(MainPanel {
  config: {
    can_become_key_window: true,
    can_become_main_window: true,
    is_floating_panel: true,
  }
});

pub fn apply_window_corner_radius(win: &tauri::WebviewWindow) -> Result<(), String> {
  let ns_window = win.ns_window().map_err(|error| error.to_string())?;
  let ns_view = win.ns_view().map_err(|error| error.to_string())?;

  let window: &NSWindow = unsafe { &*ns_window.cast() };
  let view: &NSView = unsafe { &*ns_view.cast() };

  view.setWantsLayer(true);

  let Some(layer) = view.layer() else {
    return Err("failed to access the macOS content view backing layer".into());
  };
  let layer: &CALayer = layer.as_ref();

  let clear = NSColor::clearColor();
  window.setOpaque(false);
  window.setBackgroundColor(Some(&clear));
  window.setHasShadow(true);

  layer.setCornerRadius(MACOS_WINDOW_CORNER_RADIUS);
  layer.setMasksToBounds(true);

  window.invalidateShadow();

  Ok(())
}

pub fn apply_window_panel(win: &tauri::WebviewWindow) -> Result<(), String> {
  let panel = win
    .to_panel::<MainPanel>()
    .map_err(|error| error.to_string())?;

  panel.set_level(PanelLevel::PopUpMenu.value());
  panel.set_style_mask(StyleMask::empty().borderless().nonactivating_panel().into());
  panel.set_floating_panel(true);
  panel.set_becomes_key_only_if_needed(false);
  panel.set_hides_on_deactivate(false);
  panel.set_works_when_modal(true);
  panel.set_has_shadow(true);
  panel.set_opaque(false);
  panel.set_transparent(true);
  panel.set_corner_radius(MACOS_WINDOW_CORNER_RADIUS);
  panel.set_collection_behavior(
    CollectionBehavior::new()
      .can_join_all_spaces()
      .full_screen_auxiliary()
      .ignores_cycle()
      .value(),
  );

  Ok(())
}
