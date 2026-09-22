pub fn t(key: &str) -> &str {
  match key {
    "sysenix" => "Sysenix",
    "ask_sysenix" => "Ask Sysenix...",
    "quit" => "Quit",
    "conversations_started" => "Sysenix — type your request, Enter to send, Ctrl+C to quit\n",
    "you" => "You: ",
    "thinking" => "Sysenix: thinking...",
    _ => key,
  }
}
