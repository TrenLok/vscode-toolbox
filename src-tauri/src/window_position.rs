use tauri::{PhysicalPosition, PhysicalSize, Position, Rect, WebviewWindow};

const WINDOW_PADDING: i32 = 1;

pub fn set_window_position(win: &WebviewWindow, tray_rect: Option<Rect>) -> tauri::Result<()> {
  #[cfg(target_os = "macos")]
  if let Some(tray_rect) = tray_rect {
    return set_macos_window_position(win, &tray_rect);
  }

  #[cfg(not(target_os = "macos"))]
  let _ = tray_rect;

  let Some(monitor) = win.current_monitor()? else {
    return Ok(());
  };

  let work_area = monitor.work_area();
  let metrics = WindowMetrics::from_window(win)?;

  let x = if work_area.position.x > 0 {
    work_area.position.x - metrics.shadow_horizontal()
  } else {
    work_area.position.x + work_area.size.width as i32 - metrics.outer.width
      + metrics.shadow_horizontal()
      - WINDOW_PADDING
  };

  let y = if work_area.position.y > 0 {
    work_area.position.y
  } else {
    work_area.position.y + work_area.size.height as i32 - metrics.outer.height
      + metrics.shadow_bottom()
      - WINDOW_PADDING
  };

  set_position(win, x, y)
}

#[cfg(target_os = "macos")]
fn set_macos_window_position(win: &WebviewWindow, tray_rect: &Rect) -> tauri::Result<()> {
  let metrics = WindowMetrics::from_window(win)?;

  let tray_center = tray_rect.position.to_physical::<f64>(1.0);
  let tray_size = tray_rect.size.to_physical::<f64>(1.0);
  let tray_center_x = tray_center.x + tray_size.width / 2.0;
  let tray_center_y = tray_center.y + tray_size.height / 2.0;

  let Some(monitor) = win
    .monitor_from_point(tray_center_x, tray_center_y)?
    .or(win.current_monitor()?)
  else {
    return Ok(());
  };

  let work_area = monitor.work_area();
  let scale_factor = monitor.scale_factor();
  let tray_position = tray_rect.position.to_physical::<i32>(scale_factor);
  let tray_size = tray_rect.size.to_physical::<i32>(scale_factor);

  let min_x = work_area.position.x;
  let max_x = min_x + work_area.size.width as i32 - metrics.outer.width;
  let desired_x = tray_center_x.round() as i32 - metrics.outer.width / 2;
  let x = if max_x >= min_x {
    desired_x.clamp(min_x, max_x)
  } else {
    min_x
  };

  let y = (tray_position.y + tray_size.height - metrics.shadow_top() + WINDOW_PADDING)
    .max(work_area.position.y);

  set_position(win, x, y)
}

fn set_position(win: &WebviewWindow, x: i32, y: i32) -> tauri::Result<()> {
  win.set_position(Position::Physical(PhysicalPosition { x, y }))
}

struct WindowMetrics {
  outer: PhysicalSize<i32>,
  inner: PhysicalSize<i32>,
}

impl WindowMetrics {
  fn from_window(win: &WebviewWindow) -> tauri::Result<Self> {
    Ok(Self {
      outer: to_i32_size(win.outer_size()?),
      inner: to_i32_size(win.inner_size()?),
    })
  }

  fn shadow_horizontal(&self) -> i32 {
    (self.outer.width - self.inner.width).max(0) / 2
  }

  fn shadow_bottom(&self) -> i32 {
    self.shadow_horizontal()
  }

  #[cfg(target_os = "macos")]
  fn shadow_top(&self) -> i32 {
    (self.outer.height - self.inner.height).max(0) / 2
  }
}

fn to_i32_size(size: PhysicalSize<u32>) -> PhysicalSize<i32> {
  PhysicalSize {
    width: size.width as i32,
    height: size.height as i32,
  }
}
