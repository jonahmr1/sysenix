use crate::{
  ai::{self, Result},
  click,
  constants::PYTHON_PATH,
  screenshot,
};
use std::{env, thread, time::Duration};

pub fn new(input: &str) -> Result<()> {
  let input = input.trim();
  if input.is_empty() {
    return Err("Instruction must not be empty.".into());
  }
  let executable = env::current_exe()?;
  let directory = executable
    .parent()
    .ok_or("Cannot find executable directory")?;
  let root = directory
    .ancestors()
    .find(|path| path.join(PYTHON_PATH).is_file())
    .unwrap_or(directory);
  let temporary = tempfile::tempdir()?;
  let path = temporary.path().join("screen.png");
  loop {
    let screen = screenshot::capture(&path)?;
    let Some(target) = ai::reason(root, &screen.path, input)? else {
      return Ok(());
    };
    let point = ai::ground(root, &screen.path, &target)?;
    click::click(&screen, &point)?;
    // Let the clicked UI update before capturing the next frame.
    thread::sleep(Duration::from_millis(500));
  }
}
