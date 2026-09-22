use std::io::{self, Write};

use crate::ai;
use crate::click;
use crate::locales::t;
use crate::parser;
use crate::prompt;
use crate::screenshot;
use crate::to_base64;

pub fn start() {
  println!("{}", t("conversations_started"));

  loop {
    print!("{}", t("you"));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
		
    let user_request = input.trim().to_string();

    if user_request.is_empty() {
      continue;
    }

    println!("{}", t("thinking"));
    io::stdout().flush().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));
    loop {
      let bytes = screenshot::shot();
      let image = to_base64::to_base64(&bytes);
      let prompt = prompt::system(&user_request);
      let response = ai::ask(&prompt, &image);

      print!("\r{}: ", t("sysenix"));

      if let Some(action) = parser::parse(&response) {
        println!("clicking at ({}, {})", action.x, action.y);
        click::at(action.x as f64, action.y as f64);

        std::thread::sleep(std::time::Duration::from_millis(500));
      } else {
        println!("{}", response);
        break;
      }
    }
  }
}
