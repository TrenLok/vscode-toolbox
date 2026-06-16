use std::ffi::c_void;

unsafe extern "system" {
  fn AttachThreadInput(id_attach: u32, id_attach_to: u32, attach: i32) -> i32;
  fn BringWindowToTop(hwnd: *mut c_void) -> i32;
  fn GetCurrentThreadId() -> u32;
  fn GetForegroundWindow() -> *mut c_void;
  fn GetWindowThreadProcessId(hwnd: *mut c_void, process_id: *mut u32) -> u32;
  fn IsChild(parent: *mut c_void, hwnd: *mut c_void) -> i32;
  fn SetActiveWindow(hwnd: *mut c_void) -> *mut c_void;
  fn SetForegroundWindow(hwnd: *mut c_void) -> i32;
  fn SetFocus(hwnd: *mut c_void) -> *mut c_void;
  fn SetWindowPos(
    hwnd: *mut c_void,
    hwnd_insert_after: *mut c_void,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    flags: u32,
  ) -> i32;
}

const SWP_NOMOVE: u32 = 0x0002;
const SWP_NOSIZE: u32 = 0x0001;
const HWND_TOPMOST: *mut c_void = -1isize as *mut c_void;
const HWND_NOTOPMOST: *mut c_void = -2isize as *mut c_void;

pub fn foreground_hwnd() -> *mut c_void {
  unsafe { GetForegroundWindow() }
}

pub fn is_foreground_hwnd_or_child(own_hwnd: *mut c_void) -> bool {
  let foreground_hwnd = foreground_hwnd();

  foreground_hwnd == own_hwnd || unsafe { IsChild(own_hwnd, foreground_hwnd) } != 0
}

// Uses WinAPI calls to bring a window to the foreground more reliably on Windows.
pub unsafe fn activate_window(hwnd: *mut c_void) {
  let foreground_hwnd = GetForegroundWindow();
  let foreground_thread_id = if foreground_hwnd.is_null() {
    0
  } else {
    GetWindowThreadProcessId(foreground_hwnd, std::ptr::null_mut())
  };
  let current_thread_id = GetCurrentThreadId();
  // Windows restricts foreground changes across input queues; attach briefly if needed.
  let attached = foreground_thread_id != 0
    && foreground_thread_id != current_thread_id
    && AttachThreadInput(current_thread_id, foreground_thread_id, 1) != 0;

  // Bounce through topmost to raise the window without leaving it permanently always-on-top.
  let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
  let _ = SetWindowPos(hwnd, HWND_NOTOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
  let _ = BringWindowToTop(hwnd);
  // These calls cover different parts of Windows focus/activation state.
  let _ = SetActiveWindow(hwnd);
  let _ = SetFocus(hwnd);
  let _ = SetForegroundWindow(hwnd);

  if attached {
    let _ = AttachThreadInput(current_thread_id, foreground_thread_id, 0);
  }
}
