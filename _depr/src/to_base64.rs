pub fn to_base64(bytes: &[u8]) -> String {
  const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();
  let mut i = 0;
  while i < bytes.len() {
    let b0 = bytes[i] as u32;
    let b1 = if i + 1 < bytes.len() {
      bytes[i + 1] as u32
    } else {
      0
    };
    let b2 = if i + 2 < bytes.len() {
      bytes[i + 2] as u32
    } else {
      0
    };
    result.push(CHARS[((b0 >> 2) & 0x3f) as usize] as char);
    result.push(CHARS[((b0 << 4 | b1 >> 4) & 0x3f) as usize] as char);
    result.push(if i + 1 < bytes.len() {
      CHARS[((b1 << 2 | b2 >> 6) & 0x3f) as usize] as char
    } else {
      '='
    });
    result.push(if i + 2 < bytes.len() {
      CHARS[(b2 & 0x3f) as usize] as char
    } else {
      '='
    });
    i += 3;
  }
  result
}
