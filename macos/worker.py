import json
import sys

import grounding
import reasoning


def reply(value):
	print(json.dumps(value), flush=True)


def loading(name):
	message = f"[sysenix] Loading {name}…"
	if sys.stderr.isatty():
		message = f"\x1b[36m{message}\x1b[0m"
	print(message, file=sys.stderr, flush=True)


def main():
	try:
		loading("reasoning model")
		reasoning.load_model()
		loading("grounding model")
		grounding.load_model()
	except Exception as error:
		reply({"error": str(error)})
		return 1
	reply({"ready": True})
	operations = {"reason": reasoning.reason, "ground": grounding.locate}
	for line in sys.stdin:
		try:
			request = json.loads(line)
			result = operations[request["operation"]](request["image"], request["instruction"])
			reply({"result": result})
		except Exception as error:
			reply({"error": str(error)})
	return 0


if __name__ == "__main__":
	sys.exit(main())
