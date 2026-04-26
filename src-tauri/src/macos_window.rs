use objc2_app_kit::{NSColor, NSView, NSWindow};
use objc2_quartz_core::CALayer;

const MACOS_WINDOW_CORNER_RADIUS: f64 = 8.0;

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
