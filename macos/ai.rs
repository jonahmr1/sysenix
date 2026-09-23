use crate::constants::PYTHON_PATH;
use serde::{Deserialize, de::DeserializeOwned};
use std::{
  path::Path,
  process::{Command, Stdio},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

pub fn reason(root: &Path, image: &Path, input: &str) -> Result<Option<String>> {
  infer(root, image, input, include_str!("reasoning.py"), "reason")
}

pub fn ground(root: &Path, image: &Path, target: &str) -> Result<Point> {
  infer(root, image, target, include_str!("grounding.py"), "locate")
}

fn infer<T: DeserializeOwned>(
  root: &Path,
  image: &Path,
  input: &str,
  source: &str,
  function: &str,
) -> Result<T> {
  let root = root.canonicalize()?;
  let python = root.join(PYTHON_PATH);
  let output = Command::new(if python.is_file() { python.into_os_string() } else { "python3.12".into() })
    .arg("-c")
    .arg(format!("{source}\ntry:\n print(json.dumps({function}(sys.argv[1], sys.argv[2])))\nexcept Exception as error:\n sys.exit(str(error))\n"))
    .arg(image)
    .arg(input)
    .env("SYSENIX_ROOT", root)
    .stderr(Stdio::piped())
    .output()?;
  if !output.status.success() {
    return Err(
      format!(
        "Model inference failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
      )
      .into(),
    );
  }
  Ok(serde_json::from_slice(&output.stdout)?)
}
