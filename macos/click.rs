use crate::{
  ai::{Point, Result},
  screenshot::{CGDisplayBounds, CGMainDisplayID, Position, Screenshot},
};
use std::{ffi::c_void, ptr};

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
  fn AXIsProcessTrusted() -> u8;
  fn CGEventCreateMouseEvent(
    source: *const c_void,
    event_type: u32,
    position: Position,
    button: u32,
  ) -> *const c_void;
  fn CGEventPost(tap: u32, event: *const c_void);
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
  fn CFRelease(value: *const c_void);
}

pub fn click(screenshot: &Screenshot, point: &Point) -> Result<()> {
  if point.x >= screenshot.width || point.y >= screenshot.height {
    return Err("Click coordinates are outside the screenshot.".into());
  }
  // Grounding returns image pixels; CoreGraphics expects global display points.
  let position = Position {
    x: screenshot.bounds.origin.x
      + f64::from(point.x) * screenshot.bounds.size.width / f64::from(screenshot.width),
    y: screenshot.bounds.origin.y
      + f64::from(point.y) * screenshot.bounds.size.height / f64::from(screenshot.height),
  };
  unsafe {
    if AXIsProcessTrusted() == 0 {
      return Err("Allow Accessibility in System Settings > Privacy & Security, then restart the app or terminal.".into());
    }
    if screenshot.display != CGMainDisplayID()
      || screenshot.bounds != CGDisplayBounds(screenshot.display)
    {
      return Err("Display layout changed; take another screenshot before clicking.".into());
    }
    // Create both events before posting either, so allocation failure cannot leave a button held.
    let down = CGEventCreateMouseEvent(ptr::null(), 1, position, 0);
    let up = CGEventCreateMouseEvent(ptr::null(), 2, position, 0);
    if down.is_null() || up.is_null() {
      for event in [down, up] {
        if !event.is_null() {
          CFRelease(event);
        }
      }
      return Err("Could not create mouse events.".into());
    }
    for event in [down, up] {
      CGEventPost(0, event);
      CFRelease(event);
    }
  }
  Ok(())
}
