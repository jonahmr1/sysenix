mod cli;

use std::process::ExitCode;

fn main() -> ExitCode {
  match cli::run() {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("Sysenix failed: {error}");
      ExitCode::FAILURE
    }
  }
}
