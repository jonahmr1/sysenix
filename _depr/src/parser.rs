pub struct Action {
  pub x: i32,
  pub y: i32,
}

pub fn parse(response: &str) -> Option<Action> {
  let x = extract(response, "\"x\":")?;
  let y = extract(response, "\"y\":")?;

  Some(Action {
    x: (x / 2.0) as i32,
    y: (y / 2.0) as i32,
  })
}

fn extract(s: &str, key: &str) -> Option<f64> {
  let start = s.find(key)? + key.len();
  let rest = s[start..].trim_start();
  let end = rest
    .find(|c: char| !c.is_ascii_digit() && c != '.')
    .unwrap_or(rest.len());
  rest[..end].parse().ok()
}
