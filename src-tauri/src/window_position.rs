use tauri::{Monitor, PhysicalPosition, PhysicalSize, Position, Rect, WebviewWindow};

const WINDOW_PADDING: i32 = 1;

pub fn set_window_position(win: &WebviewWindow, tray_rect: Option<Rect>) -> tauri::Result<()> {
  #[cfg(target_os = "macos")]
  if let Some(tray_rect) = tray_rect {
    return set_macos_window_position(win, &tray_rect);
  }

  #[cfg(target_os = "windows")]
  if let Some(tray_rect) = tray_rect {
    return set_windows_window_position(win, &tray_rect);
  }

  #[cfg(not(any(target_os = "macos", target_os = "windows")))]
  let _ = tray_rect;

  set_fallback_window_position(win)
}

fn set_fallback_window_position(win: &WebviewWindow) -> tauri::Result<()> {
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
  };

  set_position(win, x, y)
}

#[cfg(target_os = "windows")]
fn set_windows_window_position(win: &WebviewWindow, tray_rect: &Rect) -> tauri::Result<()> {
  let Some(monitor) = monitor_from_tray_rect(win, tray_rect)? else {
    return Ok(());
  };

  let work_area = monitor.work_area();
  let (tray_position, tray_size) = physical_tray_rect(tray_rect, monitor.scale_factor());
  let tray_center = physical_rect_center(tray_position, tray_size);
  let metrics = WindowMetrics::from_window(win)?;
  let bounds = WorkAreaBounds::from_position_size(work_area.position, work_area.size);
  let edge = nearest_tray_edge(bounds, tray_center);

  let min_x = bounds.left;
  let max_x = bounds.right - metrics.outer.width;
  let min_y = bounds.top;
  let max_y = bounds.bottom - metrics.outer.height;
  let centered_x = tray_center.x - metrics.outer.width / 2;
  let centered_y = tray_center.y - metrics.outer.height / 2;

  let x = match edge {
    TrayEdge::Left => bounds.left - metrics.shadow_horizontal(),
    TrayEdge::Right => {
      bounds.right - metrics.outer.width + metrics.shadow_horizontal() - WINDOW_PADDING
    }
    TrayEdge::Top | TrayEdge::Bottom => clamp_to_range(centered_x, min_x, max_x),
  };

  let y = match edge {
    TrayEdge::Top => bounds.top,
    TrayEdge::Bottom => bounds.bottom - metrics.outer.height + metrics.shadow_bottom(),
    TrayEdge::Left | TrayEdge::Right => clamp_to_range(centered_y, min_y, max_y),
  };

  set_position(win, x, y)
}

#[cfg(target_os = "macos")]
fn set_macos_window_position(win: &WebviewWindow, tray_rect: &Rect) -> tauri::Result<()> {
  let metrics = WindowMetrics::from_window(win)?;
  let tray_center = tray_center(tray_rect);

  let Some(monitor) = monitor_from_tray_rect(win, tray_rect)? else {
    return Ok(());
  };

  let work_area = monitor.work_area();
  let (tray_position, tray_size) = physical_tray_rect(tray_rect, monitor.scale_factor());

  let min_x = work_area.position.x;
  let max_x = min_x + work_area.size.width as i32 - metrics.outer.width;
  let desired_x = tray_center.x.round() as i32 - metrics.outer.width / 2;
  let x = clamp_to_range(desired_x, min_x, max_x);

  let y = (tray_position.y + tray_size.height - metrics.shadow_top() + WINDOW_PADDING)
    .max(work_area.position.y);

  set_position(win, x, y)
}

fn tray_center(tray_rect: &Rect) -> PhysicalPosition<f64> {
  let tray_position = tray_rect.position.to_physical::<f64>(1.0);
  let tray_size = tray_rect.size.to_physical::<f64>(1.0);

  PhysicalPosition {
    x: tray_position.x + tray_size.width / 2.0,
    y: tray_position.y + tray_size.height / 2.0,
  }
}

fn physical_rect_center(
  position: PhysicalPosition<i32>,
  size: PhysicalSize<i32>,
) -> PhysicalPosition<i32> {
  PhysicalPosition {
    x: position.x + size.width / 2,
    y: position.y + size.height / 2,
  }
}

fn monitor_from_tray_rect(win: &WebviewWindow, tray_rect: &Rect) -> tauri::Result<Option<Monitor>> {
  let tray_center = tray_center(tray_rect);

  Ok(
    win
      .monitor_from_point(tray_center.x, tray_center.y)?
      .or(win.current_monitor()?),
  )
}

fn physical_tray_rect(
  tray_rect: &Rect,
  scale_factor: f64,
) -> (PhysicalPosition<i32>, PhysicalSize<i32>) {
  (
    tray_rect.position.to_physical::<i32>(scale_factor),
    tray_rect.size.to_physical::<i32>(scale_factor),
  )
}

#[derive(Clone, Copy)]
struct WorkAreaBounds {
  left: i32,
  top: i32,
  right: i32,
  bottom: i32,
}

impl WorkAreaBounds {
  fn from_position_size(position: PhysicalPosition<i32>, size: PhysicalSize<u32>) -> Self {
    let left = position.x;
    let top = position.y;

    Self {
      left,
      top,
      right: left + size.width as i32,
      bottom: top + size.height as i32,
    }
  }
}

#[cfg(target_os = "windows")]
#[derive(Clone, Copy)]
enum TrayEdge {
  Left,
  Top,
  Right,
  Bottom,
}

#[cfg(target_os = "windows")]
fn nearest_tray_edge(bounds: WorkAreaBounds, tray_center: PhysicalPosition<i32>) -> TrayEdge {
  let distances = [
    (TrayEdge::Left, (tray_center.x - bounds.left).abs()),
    (TrayEdge::Top, (tray_center.y - bounds.top).abs()),
    (TrayEdge::Right, (bounds.right - tray_center.x).abs()),
    (TrayEdge::Bottom, (bounds.bottom - tray_center.y).abs()),
  ];

  distances
    .into_iter()
    .min_by_key(|(_, distance)| *distance)
    .map(|(edge, _)| edge)
    .unwrap_or(TrayEdge::Bottom)
}

fn clamp_to_range(value: i32, min: i32, max: i32) -> i32 {
  if max >= min {
    value.clamp(min, max)
  } else {
    min
  }
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
  fn shadow_vertical(&self) -> i32 {
    (self.outer.height - self.inner.height).max(0) / 2
  }

  #[cfg(target_os = "macos")]
  fn shadow_top(&self) -> i32 {
    self.shadow_vertical()
  }
}

fn to_i32_size(size: PhysicalSize<u32>) -> PhysicalSize<i32> {
  PhysicalSize {
    width: size.width as i32,
    height: size.height as i32,
  }
}
