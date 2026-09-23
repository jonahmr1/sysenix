import contextlib
from functools import cache
import hashlib
import json
import os
from pathlib import Path
import sys


ROOT = Path(os.environ["SYSENIX_ROOT"])
os.environ.setdefault("HF_HOME", str(ROOT / ".cache/huggingface"))
MODEL = "jonahmr1/UI-Venus-2-9B-mlx-4bit"

# https://huggingface.co/spaces/hugging-apps/ui-venus-2-9b-demo/blob/main/app.py
PROMPT = (
	"Output the center point of the position corresponding to the following instruction: \n"
	"{instruction}. \n\n"
	"The output should just be the coordinates of a point, in the format [x,y]. "
	"Additionally, if the task is infeasible (e.g., the task is not related to the image), "
	"the output should be [-1,-1]."
)


def compatible_model_path(source):
	source = Path(source).resolve()
	config_path = source / "processor_config.json"
	if not config_path.exists():
		return source

	config = json.loads(config_path.read_text())
	image_config = config.get("image_processor", {})
	if image_config.get("image_processor_type") != "Qwen3VLImageProcessor":
		return source

	image_config["image_processor_type"] = "Qwen2VLImageProcessor"
	key = hashlib.sha256(str(source).encode()).hexdigest()[:16]
	destination = ROOT / ".cache" / "compatible-models" / key
	destination.mkdir(parents=True, exist_ok=True)

	for item in source.iterdir():
		link = destination / item.name
		if (
			item.name != config_path.name
			and not link.exists()
			and not link.is_symlink()
		):
			link.symlink_to(item)

	(destination / "processor_config.json").write_text(json.dumps(config, indent=2))
	return destination


@cache
def load_model():
	with contextlib.redirect_stdout(sys.stderr):
		from mlx_vlm import load
		from mlx_vlm.utils import get_model_path
		return load(str(compatible_model_path(get_model_path(MODEL))))


def locate(path, instruction):
	instruction = instruction.strip().removesuffix(".")
	if not instruction:
		raise ValueError("Instruction must not be empty")
	with contextlib.redirect_stdout(sys.stderr):
		from PIL import Image

		with Image.open(path) as source:
			image = source.convert("RGB")
		from mlx_vlm import generate
		from mlx_vlm.prompt_utils import apply_chat_template

		model, processor = load_model()
		prompt = apply_chat_template(
			processor, model.config, PROMPT.format(instruction=instruction),
			num_images=1, enable_thinking=False,
		)
		response = generate(
			model, processor, prompt, [image],
			max_tokens=256, temperature=0.0, verbose=False,
		).text
	point = json.loads(response)
	if point == [-1, -1]:
		raise ValueError("Model could not locate the requested target")
	if (not isinstance(point, list) or len(point) != 2
		or any(type(n) not in (int, float) or not 0 <= n <= 1000 for n in point)):
		raise ValueError(f"Invalid coordinates: {response}")
	return dict(zip(("x", "y"), (min(int(n / 1000 * size), size - 1)
		for n, size in zip(point, image.size))))
