use std::{process::Command};

pub fn ask(
  prompt: &str,
  image_base64: &str,
) -> String {
  let mut messages_json = String::new();
	messages_json.push_str(&format!(
		r#"{{"role":"user","content":[{{"type":"text","text":"{}"}},{{"type":"image_url","image_url":{{"url":"data:image/png;base64,{}"}}}}]}}"#,
		prompt.replace('"', "\\\"").replace('\n', "\\n"),
		image_base64
	));

	let body = format!(
		r#"{{"model":"os-atlas","messages":[{}],"max_tokens":128}}"#,
		messages_json
	);

  let tmp = "/tmp/sysenix_request.json";
  std::fs::write(tmp, &body).unwrap();

	let output = Command::new("curl")
		.args([
			"-s",
			"-X",
			"POST",
			"http://127.0.0.1:8080/v1/chat/completions",
			"-H",
			"Content-Type: application/json",
			"-d",
			&format!("@{}", tmp),
		])
		.output()
		.unwrap();

  let raw = String::from_utf8(output.stdout).unwrap();
  println!("raw response: {}", raw);
  let json: serde_json::Value = serde_json::from_str(&raw).unwrap_or_default();
	json["choices"][0]["message"]["content"]
		.as_str()
		.unwrap_or("")
		.to_string()
}
