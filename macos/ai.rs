use crate::{constants::PYTHON_PATH, log::info};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
  io::{BufRead, BufReader, Write},
  path::Path,
  process::{Child, ChildStdout, Command, Stdio},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

pub struct Models {
  child: Child,
  output: BufReader<ChildStdout>,
}

impl Models {
  pub fn start(root: &Path) -> Result<Self> {
    let root = root.canonicalize()?;
    let python = root.join(PYTHON_PATH);
    // Separate module namespaces keep each model's settings and cache independent.
    let modules = json!({
      "reasoning": include_str!("reasoning.py"),
      "grounding": include_str!("grounding.py"),
    });
    let worker = serde_json::to_string(include_str!("worker.py"))?;
    let source = format!(
      "import sys, types\nfor name, source in {modules}.items():\n module = types.ModuleType(name)\n sys.modules[name] = module\n exec(source, module.__dict__)\nexec({worker})\n"
    );
    info("Loading models…");
    let mut child = Command::new(if python.is_file() {
      python.into_os_string()
    } else {
      "python3.12".into()
    })
    .args(["-u", "-c", &source])
    .env("SYSENIX_ROOT", root)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::inherit())
    .spawn()?;
    let mut models = Self {
      output: BufReader::new(child.stdout.take().expect("piped stdout")),
      child,
    };
    if models.read()?["ready"] != true {
      return Err("Model worker did not signal readiness.".into());
    }
    info("Models ready.");
    Ok(models)
  }

  pub fn reason(&mut self, image: &Path, input: &str) -> Result<Option<String>> {
    self.infer("reason", image, input)
  }

  pub fn ground(&mut self, image: &Path, target: &str) -> Result<Point> {
    self.infer("ground", image, target)
  }

  fn infer<T: DeserializeOwned>(
    &mut self,
    operation: &str,
    image: &Path,
    input: &str,
  ) -> Result<T> {
    let request = json!({"operation": operation, "image": image, "instruction": input});
    let input = self
      .child
      .stdin
      .as_mut()
      .ok_or("Model worker input is closed")?;
    writeln!(input, "{request}")?;
    input.flush()?;
    let response = self.read()?;
    let result = response.get("result").ok_or("Missing model result")?;
    Ok(serde_json::from_value(result.clone())?)
  }

  fn read(&mut self) -> Result<Value> {
    let mut line = String::new();
    if self.output.read_line(&mut line)? == 0 {
      return Err("Model worker exited unexpectedly; see its stderr output.".into());
    }
    let response: Value = serde_json::from_str(&line)?;
    if let Some(error) = response.get("error") {
      return Err(format!("Model inference failed: {error}").into());
    }
    Ok(response)
  }
}

impl Drop for Models {
  fn drop(&mut self) {
    drop(self.child.stdin.take());
    let _ = self.child.wait();
  }
}
