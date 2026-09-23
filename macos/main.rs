use std::{
  env,
  io::{self, Write},
  process::{Command, ExitCode, Stdio},
};

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
  let args: Vec<_> = env::args_os().skip(1).collect();
  if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
    println!("Usage: sysenix <image> <instruction>");
    return Ok(ExitCode::SUCCESS);
  }
  if args.len() != 2 {
    return Err("Usage: sysenix <image> <instruction>".into());
  }
  let executable = env::current_exe()?;
  let directory = executable
    .parent()
    .ok_or("Cannot find executable directory")?;
  let root = directory
    .ancestors()
    .find(|path| path.join(".venv/bin/python").is_file())
    .unwrap_or(directory);
  let python = root.join(".venv/bin/python");
  let output = Command::new(if python.is_file() { python.into_os_string() } else { "python3.12".into() })
        .arg("-c")
        .arg(concat!(include_str!("grounding.py"), "\ntry:\n print(json.dumps(locate(sys.argv[1], sys.argv[2])))\nexcept Exception as error:\n sys.exit(str(error))\n"))
        .args(&args)
        .env("SYSENIX_ROOT", root)
        .stderr(Stdio::inherit())
        .output()?;
  if !output.status.success() {
    return Err("Grounding failed".into());
  }
  io::stdout().write_all(&output.stdout)?;
  Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
  match run() {
    Ok(status) => status,
    Err(error) => {
      eprintln!("Sysenix failed: {error}");
      ExitCode::FAILURE
    }
  }
}
