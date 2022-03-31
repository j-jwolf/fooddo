import os, sys
from ast import literal_eval
from datetime import datetime

fn = "verify.fdo"
if(not os.path.isfile(fn)): sys.exit()
with open(fn) as file:
	try:
		data = literal_eval(file.read())
		if(type(data) != type(dict)):
			...
			sys.exit()
	except KeyboardInterrupt: sys.exit()
	except Exception as e:
		...
		sys.exit()

if(len(sys.argv) != 3):
	...
	sys.exit()
email = sys.argv[1]
code = sys.argv[2]
match = False
if(data[email] == code):
	match = True
	data.pop(email)
	if(len(data.keys()) == 0): os.remove(fn)
	else:
		with open(fn, "w") as file: file.write(str(data))
with open("match.fdo", "w") as file: file.write(f"{email},{match}")
