import json
import sys

import grounding
import reasoning


def reply(value):
	print(json.dumps(value), flush=True)


def main():
	try:
		reasoning.load_model()
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
