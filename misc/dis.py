#!/usr/bin/python3
from question import *
from sys import argv
import re

ENDIAN = "big"
HEADER = b"MQS-OBJ-V0.0.1\0"

questions: list = []
strings: list = []

def err(msg):
	print("Error:", msg)
	exit(1)

def dis(file):
	data = bytes()
	bi = 0
	def readb(sz = 1):
		nonlocal data, bi
		bi += sz
		if bi >= dsize: err("Missing data")
		return int.from_bytes(data[bi - sz:bi], byteorder=ENDIAN)

	# ============ input ============
	with open(file, "rb") as f: data = f.read()
	dsize = len(data)

	# ============ header ============
	if dsize < len(HEADER) or data[0:len(HEADER)] != HEADER:
		err("Invalid mqs object header")
	bi = len(HEADER)
	# check checksum
	if readb(readb()) != sum(data[bi:]): err("Invalid checksum")

	# ============ info ============
	stris = readb()
	qis = readb()
	qcount = readb(qis)

	# ============ questions ===========
	for i in range(qcount):
		if i != readb(qis): err("Invalid question index")
		name = readb(stris)
		theory = readb(stris)
		sis = readb()
		scount = readb(sis)
		steps = []

		for si in range(scount):
			if si != readb(sis): err("Invalid step index")
			steps.append(Step(
				readb(stris), # description
				readb(stris), # theory
				readb(stris), # before
				readb(stris), # after
			))

		concl = readb(stris) # conclusion
		ans = readb(stris) # answer
		anst = readb(1) # answer type

		stried = readb(readb()) # steps tried

		questions.append(Question(
			name, theory, steps, concl, ans, anst, stried))

	# ============ strings ===========
	while bi < dsize:
		if readb() != 0: err("Expected null-byte")
		if readb(stris) != len(strings): err("Invalid string index")
		
		l = 0
		while bi + l < dsize and data[bi + l] != 0: l += 1
		strings.append(data[bi:bi+l].decode("UTF-8"))
		bi += l

	# print("strings:")
	# for i, s in zip(range(len(strings)), strings):
	# 	print(f"  {i}: \"{s}\"")

def print_question(q):
	print(f"question: ?{strings[q.name]}")
	print(f"    theory: `{strings[q.theory]}`")

	print("    approach:")
	for si, s in zip(range(len(q.steps)), q.steps):
		print(f"        {si + 1}: {strings[s.description]}")
		print("            " + strings[s.process])

	print(f"        {strings[q.conclusion]}")	
	print(f"    answer: {strings[q.answer]} ({'true' if q.answer_type else 'false'})")	
	print("    steps tried:", q.steps_tried)

def print_all():
	correct = 0
	for q in questions:
		print_question(q)
		print()
		if q.answer_type: correct += 1


	print(f"{correct}/{len(questions)} answers are true\n")
	print("re-run with '--at=QUESTION[:STEP]' to see the state of a question at the given step")

def print_at(name, step):
	for q in questions:
		if strings[q.name] == name:
			question = q
			break
	else: err(f"Question '{name}' doesn't exist")
	if step < 1 or step > len(q.steps): err("Invalid step number")

	print(f"question: ?{strings[q.name]} (step {step})")
	print(f"    theory: `{strings[q.theory]}`")

	s = q.steps[step - 1]

	print("    state before step:")	
	print(f"        `{strings[s.before]}`")

	print("    step:", strings[s.description])	
	print(f"        {strings[s.process]}")

	print("    state after step:")	
	print(f"        `{strings[s.after]}`")

if __name__ == '__main__':
	dis("obj.o")

	for s in argv:
		if s.startswith("--at"):
			m = re.compile(r'\s*=\s*([a-zA-Z_][a-zA-Z0-9_]*)(?::([0-9]+))?').match(s.strip("--at"))
			if not m: err("Invalid argument")

			if m.group(2): print_at(m.group(1), int(m.group(2)))
			else:
				for q in questions:
					if strings[q.name] == m.group(1):
						print_question(q)
						break
				else: err(f"Question '{m.group(1)}' doesn't exist")
			break

	else: print_all()