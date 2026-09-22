use std::os::raw::c_double;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
  fn CGEventCreateMouseEvent(
    source: *const u8,
    mouse_type: u32,
    mouse_cursor_position: CGPoint,
    mouse_button: u32,
  ) -> *mut u8;
  fn CGEventPost(tap: u32, event: *mut u8);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGPoint {
  x: c_double,
  y: c_double,
}

pub fn at(x: f64, y: f64) {
  unsafe {
    let point = CGPoint { x, y };
    let event = CGEventCreateMouseEvent(std::ptr::null(), 1, point, 0); // 1 = left mouse down
    CGEventPost(0, event);
    let event = CGEventCreateMouseEvent(std::ptr::null(), 2, point, 0); // 2 = left mouse up
    CGEventPost(0, event);
  }
}
