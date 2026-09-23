use std::{env, path::Path};
use sysenix::{
  ai::{self, Result},
  constants::PYTHON_PATH,
  request,
};

const USAGE: &str =
  "Usage: sysenix \"<instruction>\"\n       sysenix reason <image> \"<instruction>\"";

pub fn run() -> Result<()> {
  let args: Vec<_> = env::args_os().skip(1).collect();
  if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
    println!("{USAGE}");
    return Ok(());
  }
  if args.len() == 3 && args[0] == "reason" {
    let input = args[2].to_str().ok_or("Instruction must be valid UTF-8")?;
    let executable = env::current_exe()?;
    let directory = executable
      .parent()
      .ok_or("Cannot find executable directory")?;
    let root = directory
      .ancestors()
      .find(|path| path.join(PYTHON_PATH).is_file())
      .unwrap_or(directory);
    let target = ai::reason(root, Path::new(&args[1]), input)?;
    println!("{}", serde_json::to_string(&target)?);
    return Ok(());
  }
  if args.len() != 1 {
    return Err(USAGE.into());
  }
  request::new(args[0].to_str().ok_or("Instruction must be valid UTF-8")?)
}
