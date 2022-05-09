class Step:
	def __init__(self, d, p, b, a):
		self.description: int = d
		self.process: int = p
		self.before: int = b
		self.after: int = a

class Question:
	def __init__(self, n, t, s, c, a, at, st):
		self.name: int = n
		self.theory: int = t
		self.steps: list = s
		self.conclusion: int = c
		self.answer: int = a
		self.answer_type: bool = at
		self.steps_tried: int = st