use std::{
  fmt::Display,
  io::{self, IsTerminal},
};

pub fn info(message: impl Display) {
  if io::stderr().is_terminal() {
    eprintln!("\x1b[36m[sysenix] {message}\x1b[0m");
  } else {
    eprintln!("[sysenix] {message}");
  }
}
