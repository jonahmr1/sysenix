import argparse
import contextlib
import hashlib
import json
import math
import os
from pathlib import Path
import re
import sys

# Model source and local download cache.
ROOT = Path(__file__).resolve().parents[1]
os.environ.setdefault("HF_HOME", str(ROOT / ".cache/huggingface"))
MODEL = "jonahmr1/UI-Venus-2-9B-mlx-4bit"

# Ask for the target center, or [-1,-1] when it cannot be located.
# Prompt from the Venus demo:
# https://huggingface.co/spaces/hugging-apps/ui-venus-2-9b-demo/blob/main/app.py
PROMPT = (
	"Output the center point of the position corresponding to the following instruction: \n"
	"{instruction}. \n\n"
	"The output should just be the coordinates of a point, in the format [x,y]. "
	"Additionally, if the task is infeasible (e.g., the task is not related to the image), "
	"the output should be [-1,-1]."
)


# Adapt legacy processor metadata so the downloaded model can load.
# Reuse the existing weights and preserve image-processing parameters.
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
		if item.name == "processor_config.json":
			continue
		link = destination / item.name
		if not link.exists() and not link.is_symlink():
			link.symlink_to(item)
	(destination / "processor_config.json").write_text(json.dumps(config, indent=2))
	return destination


# Validate the returned point or box, then convert its center from 0–1000
# coordinates to original-image pixels. This does not verify visual accuracy.
def pixel_point(response, width, height):
	text = response.strip()
	try:
		values = json.loads(text)
	except json.JSONDecodeError:
		match = re.fullmatch(
			r"<point>\s*(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s*</point>", text
		)
		if not match:
			raise ValueError(f"Invalid coordinate response: {text!r}")
		values = [float(item) for item in match.groups()]
	if not isinstance(values, list):
		raise ValueError("Expected a point or bounding box")
	if len(values) == 2 and all(isinstance(v, list) and len(v) == 2 for v in values):
		values = values[0] + values[1]
	if len(values) not in (2, 4) or any(
		type(v) not in (int, float) or not math.isfinite(v) for v in values
	):
		raise ValueError("Invalid coordinate numbers")
	if values == [-1, -1]:
		raise ValueError("Model could not locate the requested target")
	if any(v < 0 or v > 1000 for v in values):
		raise ValueError("Coordinates outside the expected 0–1000 range")
	if len(values) == 4:
		x1, y1, x2, y2 = values
		if x1 > x2 or y1 > y2:
			raise ValueError("Reversed bounding box")
		values = [(x1 + x2) / 2, (y1 + y2) / 2]
	x, y = values

	pixel = {
		"x": min(int(x / 1000 * width), width - 1),
		"y": min(int(y / 1000 * height), height - 1),
	}
	# Click coordinates here assume two screenshot pixels per screen point.
	return {
		"pixel.x": pixel["x"],
		"pixel.y": pixel["y"],
		"click.x": pixel["x"] / 2,
		"click.y": pixel["y"] / 2,
	}


def main():
	# 1. Read and validate the screenshot path and target instruction.
	parser = argparse.ArgumentParser(
		description="Find a target's original-image pixel coordinates locally."
	)
	parser.add_argument("image", type=Path)
	parser.add_argument("instruction")
	parser.add_argument(
		"--model", default=MODEL, help="MLX model repository or local directory"
	)
	args = parser.parse_args()
	if not args.image.is_file():
		parser.error(f"Image does not exist: {args.image}")
	instruction = args.instruction.strip().removesuffix(".")
	if not instruction:
		parser.error("Instruction must not be empty")

	try:
		with contextlib.redirect_stdout(sys.stderr):
			from PIL import Image
			from mlx_vlm import generate, load
			from mlx_vlm.prompt_utils import apply_chat_template
			from mlx_vlm.utils import get_model_path

			# 2. Read the screenshot and retain its original dimensions.
			with Image.open(args.image) as source:
				image = source.convert("RGB")
			print(
				f"Loading {args.model}; image {image.width}×{image.height}",
				file=sys.stderr,
			)
			# 3. Find or download the model, apply compatibility fixes, and load it.
			model_path = compatible_model_path(get_model_path(args.model))
			model, processor = load(str(model_path))
			# 4. Ask Venus to locate the target in the image using local inference.
			prompt = apply_chat_template(
				processor,
				model.config,
				PROMPT.format(instruction=instruction),
				num_images=1,
				enable_thinking=False,
			)
			result = generate(
				model,
				processor,
				prompt,
				[image],
				max_tokens=256,
				temperature=0.0,
				verbose=False,
			)
			# 5. Validate the answer and convert it into image-pixel coordinates.
			point = pixel_point(result.text, image.width, image.height)
		# 6. Return JSON on success, or report an error on failure. No mouse click.
		print(json.dumps(point))
	except Exception as error:
		print(f"Grounding failed: {error}", file=sys.stderr)
		return 1
	return 0


if __name__ == "__main__":
	sys.exit(main())
