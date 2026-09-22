pub fn system(user_request: &str) -> String {
  format!(
    r#"You are a UI grounding model.
Given this screenshot, locate the UI element matching this instruction:
"{}"

Return only valid JSON with this exact shape:
{{"action":"click","x":number,"y":number,"confidence":number}}

Rules:
- Coordinates must be image pixel coordinates from the top-left corner.
- Click the center of the target, not an edge or label.
- Do not include markdown, explanation, or extra text."#,
    user_request
  )
}
