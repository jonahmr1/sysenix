use std::{env, path::Path};
use sysenix::{
  ai::{self, Result},
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
    let mut models = ai::Models::start()?;
    let target = models.reason(Path::new(&args[1]), input)?;
    println!("{}", serde_json::to_string(&target)?);
    return Ok(());
  }
  if args.len() != 1 {
    return Err(USAGE.into());
  }
  let input = args[0].to_str().ok_or("Instruction must be valid UTF-8")?;
  request::new(&mut ai::Models::start()?, input)
}
