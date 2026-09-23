use std::env;
use sysenix::{ai::Result, request};

const USAGE: &str = "Usage: sysenix \"<instruction>\"";

pub fn run() -> Result<()> {
  let args: Vec<_> = env::args_os().skip(1).collect();
  if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
    println!("{USAGE}");
    return Ok(());
  }
  if args.len() != 1 {
    return Err(USAGE.into());
  }
  request::new(args[0].to_str().ok_or("Instruction must be valid UTF-8")?)
}
