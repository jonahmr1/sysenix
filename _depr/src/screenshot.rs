use std::process::Command;

pub fn shot() -> Vec<u8> {
  let path = "/tmp/sysenix_screenshot.png";
  Command::new("screencapture")
    .arg("-x") // no sound
    .arg("-m") // main screen only
    .arg(path)
    .output()
    .unwrap();
  std::fs::read(path).unwrap()
}
