use crate::{new_formatted_error, get_cli_arg};

pub type StringCollection = Vec<String>;
pub type StringIndex = usize;

static TAB: &str = "    ";

pub struct Step<T> {
	pub description: T,
	pub process: T,
	pub state_before: T,
	pub state_after: T,
}

pub struct Question<T> {
	pub name: T,
	pub theory: T,
	pub steps: Vec<Step<T>>,
	pub conclusion: T,
	pub answer: T,
	pub is_true: bool,
	pub steps_tried: u64,
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

// normal printing
impl SQuestion {
	fn print_normal(&self) {
		println!("question: ?{}", self.name);
		println!("{}theory: `{}`", TAB, self.theory);
		println!("{}approach:", TAB);

		for (i, s) in self.steps.iter().enumerate() {
			println!("{}{}{}: {}", TAB, TAB, i + 1, s.description);
			println!("{}{}{}`{}`", TAB, TAB, TAB, s.process);
		}
		
		println!("{}{}{}", TAB, TAB, self.conclusion);
		println!("{}answer: {} ({})", TAB, self.answer, self.is_true);
		println!("{}steps tried: {}", TAB, self.steps_tried);
	}

	fn print_at_normal(&self, step: usize) {
		println!("question: ?{} (step {})", self.name, step);
		println!("{}theory: `{}`", TAB, self.theory);

		let step = &self.steps[step - 1];

		println!("{}state before step:", TAB);
		println!("{}{}`{}`", TAB, TAB, step.state_before);

		println!("{}step: {}", TAB, step.description);
		println!("{}{}{}", TAB, TAB, step.process);

		println!("{}state after step:", TAB);
		println!("{}{}`{}`", TAB, TAB, step.state_after);
	}
}

// markdown printing
impl SQuestion {
	fn print_markdown(&self) {
		println!("### question: ?{}", self.name);
		println!("&emsp;theory: `{}` \\", self.theory);
		println!("&emsp;approach: \\");

		for (i, s) in self.steps.iter().enumerate() {
			println!("&emsp;&emsp;{}: {} \\", i + 1, s.description);
			println!("&emsp;&emsp;&emsp;`{}` \\", s.process);
		}
		
		println!("&emsp;&emsp;{} \\", self.conclusion);
		println!("&emsp;answer: {} ({}) \\", self.answer, self.is_true);
		println!("&emsp;steps tried: {}", self.steps_tried);
	}

	fn print_at_markdown(&self, step: usize) {
		println!("### question: ?{} (step {})", self.name, step);
		println!("&emsp;theory: `{}` \\", self.theory);

		let step = &self.steps[step - 1];

		println!("&emsp;state before step: \\");
		println!("&emsp;&emsp;`{}` \\", step.state_before);

		println!("&emsp;step: {} \\", step.description);
		println!("&emsp;&emsp;{} \\", step.process);

		println!("&emsp;state after step: \\");
		println!("&emsp;&emsp;`{}`", step.state_after);
	}
}

// printing
impl SQuestion {
	pub fn print(&self) {
		if get_cli_arg!(markdown) {
			self.print_markdown();
		} else {
			self.print_normal();
		}
	}

	pub fn print_at(&self, step: usize) {
		if step < 1 || step > self.steps.len() {
			new_formatted_error!(InvalidStepNumber step, self.steps.len()).dispatch();
			crate::exit(1);
		}

		if get_cli_arg!(markdown) {
			self.print_at_markdown(step);
		} else {
			self.print_at_normal(step);
		}
	}
}