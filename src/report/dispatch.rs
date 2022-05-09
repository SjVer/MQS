use crate::info::report::CODE_PREFIX;
use crate::lex::span::Span;
use yansi::Color;

pub trait Report {
	
}

// pub fn dispatch_header(color: Color, label: &str, code: u8, message: &String) -> () {
// 	//! Dispatches a header in the form of "label: message" or "label [code]: message"

// 	let label = if code != 0 { format!("{}[{}{:#04}]", label, CODE_PREFIX, code) }
// 				else { String::from(label) };

// 	println!("{}{}", color.paint(label).bold(), Color::Unset.paint(format!(": {}", message)).bold());
// }

// pub fn dispatch_snippet(color: Option<Color>, span: &Span) -> () {
// 	//! Dispatches a snippet annonation in the form of "  file:line:col"

// 	println!("  {}", Color::Cyan.paint(&span.start).bold());

// 	// temporary:
// 	print!("{}", Color::Cyan.paint("    │ ").bold());
// 	println!("var a = 123;");

// 	print!("{}", Color::Cyan.paint(format!(" {} │ ", span.start.line)).bold());
// 	print!("func f(firstparam, ");
// 	if let Some(color) = color { print!("{}", color.paint("firstparam").bold()); }
// 	else { print!("firstparam"); }
// 	println!(") {{");

// 	print!("{}", Color::Cyan.paint("    ╵                    ").bold());
// 	print!("{}", Color::Red.paint("^~~~~~~~~~").bold());
// 	println!("\n");
// }