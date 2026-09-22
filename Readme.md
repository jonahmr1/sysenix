# Sysenix
Finally your computer agent that does everything for you without you having to think about it. you ask, and it does **IN THE RIGHT WAY**.

## What does it include?
- **AI layer** — natural language interface on top
- Controller - controls your device by clicking, typing and watching
- Tracker — watches your system, logs every change
- Executor — undoes changes cleanly and completely

## How does it work?
- While it contains AI, Tracker and an Executor, this will have the ability to do something like this:
You tell it:
> clean up everything related to **Intellenix**

And it knows exactly what to remove — every file, every path, every config — because it's been watching since day one.
Or:
> what did I install last week after I asked you about **Intellenix**?

And it gives you a full list with locations.
Or instead of asking AI how to install **Intellenix** and going through 10 steps of errors, you just say:
> install **Intellenix**

And it figures out the right way **BASED ON your specific system**, does it, and logs every single change so you can undo it later with one command.

You ask:
> create an Intellenix account for me
And it'll open app/browser and process the request by interacting from your device

## Guide

Prerequisites:

- An Apple Silicon Mac (M1 or newer); the current setup has been tested on an M3 with 24 GB RAM.
- Python 3.12 and Git (to clone the repository).
- Internet access for installing dependencies and downloading the model.
- About 7 GB for the model and Python environment, plus extra space during installation.
- VS Code with the Python extension.

Run these commands from the project root. Rust and Cargo are not required.

### Initialize

```bash
python3.12 -m venv .venv
.venv/bin/python -m pip install -r ground/dep.txt
```

In VS Code, install the Python extension, then press **⌘⇧P → Python: Select Interpreter** and choose `.venv/bin/python`.

Activate the environment in each new terminal to use `python` directly:

```bash
source .venv/bin/activate
```

Download the model separately, from the project directory:

```bash
HF_HOME="$PWD/.cache/huggingface" .venv/bin/hf download jonahmr1/UI-Venus-2-9B-mlx-4bit
```

### Test

```bash
python ground/main.py "/path/to/screenshot.png" "click the ..."
```
