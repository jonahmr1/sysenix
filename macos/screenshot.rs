use crate::ai::Result;
use std::{
  fs::File,
  io::Read,
  path::{Path, PathBuf},
  process::Command,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
  pub x: f64,
  pub y: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
  pub width: f64,
  pub height: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
  pub origin: Position,
  pub size: Size,
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
  pub(crate) fn CGMainDisplayID() -> u32;
  pub(crate) fn CGDisplayBounds(display: u32) -> Bounds;
  fn CGPreflightScreenCaptureAccess() -> bool;
}

pub struct Screenshot {
  pub path: PathBuf,
  pub width: u32,
  pub height: u32,
  pub display: u32,
  pub bounds: Bounds,
}

pub fn capture(path: &Path) -> Result<Screenshot> {
  if !unsafe { CGPreflightScreenCaptureAccess() } {
    return Err("Allow Screen Recording in System Settings > Privacy & Security, then restart the app or terminal.".into());
  }
  let display = unsafe { CGMainDisplayID() };
  let bounds = unsafe { CGDisplayBounds(display) };
  let path = std::env::current_dir()?.join(path);
  let output = Command::new("/usr/sbin/screencapture")
    .args(["-x", "-m", "-t", "png"])
    .arg(&path)
    .output()?;
  if !output.status.success() {
    return Err(
      format!(
        "Screenshot failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
      )
      .into(),
    );
  }
  if display != unsafe { CGMainDisplayID() } || bounds != unsafe { CGDisplayBounds(display) } {
    return Err("Display layout changed during capture; take another screenshot.".into());
  }
  // PNG's IHDR stores the actual image dimensions, including Retina scaling.
  let mut header = [0; 24];
  File::open(&path)?.read_exact(&mut header)?;
  if &header[..8] != b"\x89PNG\r\n\x1a\n" || &header[12..16] != b"IHDR" {
    return Err("Screenshot is not a PNG image.".into());
  }
  let width = u32::from_be_bytes(header[16..20].try_into()?);
  let height = u32::from_be_bytes(header[20..24].try_into()?);
  if width == 0 || height == 0 || bounds.size.width <= 0.0 || bounds.size.height <= 0.0 {
    return Err("Screenshot has invalid dimensions.".into());
  }
  Ok(Screenshot {
    path,
    width,
    height,
    display,
    bounds,
  })
}
