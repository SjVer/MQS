#!/usr/bin/python3
from question import *

ENDIAN = "big"
HEADER = b"MQS-OBJ-V0.0.1\0"

def sizeof(val): return max((val.bit_length() + 7) // 8, 1)

questions = [
	# theory, [steps]
	Question(0, 1, [Step(2, 3, 4, 5)], 6, 7, True, 803),
]
qsize = sizeof(len(questions) - 1)

strings = [
	"my_question", # 0
	"0 * x = 0", # 1
	"rewrite using std::!mul_zero", # 2
	"`0 * ..` -> `0`", # 3
	"0 * x = 0", # 4
	"0 = 0", # 5
	"lhs matches rhs! (std::!sides_equal)", # 6
	"correct", # 7
]
ssize = sizeof(len(strings) - 1)


def comp(outfile):
	data = bytearray()
	def b(value, sz):
		nonlocal data
		data += value.to_bytes(sz, ENDIAN)

	# ======= info =======
	b(ssize, 1) # string index size
	b(qsize, 1) # question index size
	b(len(questions), qsize) # question count

	# ====== questions ======
	for i, q in zip(range(len(questions)), questions):
		b(i, qsize) # index
		b(q.name, ssize) # name
		b(q.theory, ssize) # theory

		stepsize = sizeof(len(q.steps) - 1)
		b(1, stepsize) # step index size
		b(len(q.steps), stepsize) # step count

		for si, s in zip(range(len(q.steps)), q.steps):
			b(si, stepsize) # index
			b(s.description, ssize) # description
			b(s.process, ssize) # process
			b(s.before, ssize) # before
			b(s.after, ssize) # after

		b(q.conclusion, ssize) # conclusion
		b(q.answer, ssize) # answer
		b(q.answer_type, 1) # answer type

		b(sizeof(q.steps_tried), 1) # steps tried size
		b(q.steps_tried, sizeof(q.steps_tried)) # steps tried

	# ====== strings ======
	for i, s in zip(range(len(strings)), strings):
		data += b'\0'
		b(i, ssize) # index
		data += bytes(s, "UTF-8") # string

	# ====== header ======
	cs = sum(data)
	data = HEADER \
		+ sizeof(cs).to_bytes(1, ENDIAN) \
		+ cs.to_bytes(sizeof(cs), ENDIAN) \
		+ data

	# ====== output ======
	with open(outfile, "wb") as f:
		f.write(data)



if __name__ == '__main__':
	comp("obj.o")