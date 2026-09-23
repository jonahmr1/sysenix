mod cli;

use std::process::ExitCode;

fn main() -> ExitCode {
  match cli::run() {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      sysenix::log::info(format_args!("Failed: {error}"));
      ExitCode::FAILURE
    }
  }
}
