use crate::{
  ai::{self, Result},
  click,
  constants::PYTHON_PATH,
  log::info,
  screenshot,
};
use std::{
  env, thread,
  time::{Duration, Instant},
};

pub fn new(input: &str) -> Result<()> {
  let input = input.trim();
  if input.is_empty() {
    return Err("Instruction must not be empty.".into());
  }
  info(format_args!("Request: {input}"));
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
  let mut models = ai::Models::start(root)?;
  loop {
    info("Taking a screenshot ...");
    let screen = screenshot::capture(&path)?;
    info("Reasoning ...");
    let start = Instant::now();
    let result = models.reason(&screen.path, input);
    info(format_args!(
      "Reasoning took {:.2}s",
      start.elapsed().as_secs_f64()
    ));
    let Some(target) = result? else {
      info("Reasoning returned null. Stopping.");
      return Ok(());
    };
    info(format_args!("Target: {target}"));
    info("Grounding ...");
    let start = Instant::now();
    let result = models.ground(&screen.path, &target);
    info(format_args!(
      "Grounding took {:.2}s",
      start.elapsed().as_secs_f64()
    ));
    let point = result?;
    info(format_args!(
      "Click image coordinates ({}, {})…",
      point.x, point.y
    ));
    click::click(&screen, &point)?;
    info("Executing a 2s delay for the screen to update…");
    // Let the clicked UI update before capturing the next frame.
    thread::sleep(Duration::from_millis(2000));
  }
}
