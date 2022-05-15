pub type StringCollection = Vec<String>;
pub type StringIndex = usize;

pub struct Step<T> {
	description: T,
	process: T,
	state_before: T,
	state_after: T,
}

pub struct Question<T> {
	name: T,
	theory: T,
	steps: Vec<Step<T>>,
	conclusion: T,
	answer: T,
	is_true: bool,
	steps_tried: u32,
}

pub type IStep = Step<StringIndex>;
pub type IQuestion = Question<StringIndex>;
pub type SStep = Step<String>;
pub type SQuestion = Question<String>;

impl IStep {
	pub fn stringify(&self, strings: &StringCollection) -> SStep {
		SStep {
			description: strings[self.description].clone(),
			process: strings[self.process].clone(),
			state_before: strings[self.state_before].clone(),
			state_after: strings[self.state_after].clone(),
		}
	}
}

impl IQuestion {
	pub fn stringify(&self, strings: &StringCollection) -> SQuestion {
		// stringify steps
		let mut steps = Vec::<SStep>::new();
		for step in &self.steps {
			steps.push(step.stringify(strings));
		}

		SQuestion {
			name: strings[self.name].clone(),
			theory: strings[self.theory].clone(),
			steps,
			conclusion: strings[self.conclusion].clone(),
			answer: strings[self.answer].clone(),
			is_true: self.is_true,
			steps_tried: self.steps_tried,
		}
	}
}