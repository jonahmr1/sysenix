use crate::ai::{Models, Result};
use std::{
  ffi::{CStr, c_char},
  sync::Mutex,
};

static MODELS: Mutex<Option<Models>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn sysenix_start() {
  run(|| {
    let mut models = MODELS.lock().map_err(|_| "Model worker lock poisoned")?;
    if models.is_none() {
      *models = Some(Models::start()?);
    }
    Ok(())
  });
}

// The caller keeps a valid, NUL-terminated UTF-8 string alive until this returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysenix_request(input: *const c_char) {
  run(|| {
    if input.is_null() {
      return Err("Missing request input".into());
    }
    let input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let mut models = MODELS.lock().map_err(|_| "Model worker lock poisoned")?;
    crate::request::new(models.as_mut().ok_or("Models are not loaded")?, input)
  });
}

fn run(action: impl FnOnce() -> Result<()> + std::panic::UnwindSafe) {
  match std::panic::catch_unwind(action) {
    Ok(Ok(())) => {}
    Ok(Err(error)) => crate::log::info(error),
    Err(_) => crate::log::info("Rust operation panicked"),
  }
}
