# Sysenix

Give an image and an instruction to Venus/MLX; receive `{ "x": ..., "y": ... }`
in original-image pixels. Requires Apple Silicon macOS, Rust, and Python 3.12.

```bash
python3.12 -m venv .venv
.venv/bin/python -m pip install -r macos/dep.txt
cargo run --release -- "/path/to/image.png" "locate the battery icon"
```

The model downloads on first use and reuses the repository's existing
`.cache/huggingface` cache. A configured `HF_HOME` is respected.
Errors go to stderr with a nonzero exit status.

Rust handles the CLI and prints the result. Python only processes the image with
the model and returns coordinates. Its source is embedded in the Rust binary.
The binary finds `.venv/bin/python` beside itself or in a parent directory,
falling back to `python3.12` on PATH. For delivery, provision `.venv` beside the
binary; that directory also holds the model cache.
