# Sysenix

Sysenix is a local AI companion for your computer, like a friend sitting beside you. When you need help, ask naturally, and it works through the task with you.

For macOS setup, use an Apple Silicon Mac with Rust and Python 3.12 installed:

```bash
python3.12 -m venv .venv
.venv/bin/python -m pip install -r macos/dep.txt
HF_HOME="$PWD/.cache/huggingface" .venv/bin/hf download jonahmr1/UI-Venus-2-9B-mlx-4bit
HF_HOME="$PWD/.cache/huggingface" .venv/bin/hf download mlx-community/Qwen3.5-9B-4bit
```

Once both downloads finish, run:

```bash
cd sysenix/macos
cargo run -- "create an apple account"
```
