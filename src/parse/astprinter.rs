use super::ast::*;
use crate::lex::token::TokenKind::{self, *};

macro_rules! normal_or_md {
	($normal:expr, $md:expr) => {
		if crate::get_cli_arg!(markdown) { $md }
		else { $normal }
	}
}

// ================ Theory ================

pub struct TheoryPrinter;

impl TheoryVisitor<String> for TheoryPrinter {
	fn visit_logical(&mut self, node: &TheoryNode, lhs: &TheoryNode, rhs: &TheoryNode) -> String {
		let this = match node.token.kind {
			Or => normal_or_md!("|", "\\operatorname{|}"),
			And => normal_or_md!("&", "\\operatorname{\\&}"),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_unary(&mut self, node: &TheoryNode, expr: &TheoryNode) -> String {
		let this = match node.token.kind {
			Not => normal_or_md!("~", "\\neg"),
			_ => unreachable!(),
		};

		format!("{} {}", this, self.visit(expr))
	}

	fn visit_implies(&mut self, node: &TheoryNode, lhs: &TheoryNode, rhs: &TheoryNode) -> String {
		let this = match node.token.kind {
			Implies => normal_or_md!("<=>", "\\iff"),
			// NotImplies => normal_or_md!("<!>", "\\;\\;\\;\\not\\!\\!\\!\\!\\!\\iff"),
			NotImplies => normal_or_md!("<!>", "\\centernot\\iff"),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_comparison(&mut self, node: &TheoryNode, lhs: &TheoryNode, rhs: &TheoryNode) -> String {
		let this = match node.token.kind {
			DefEquals => normal_or_md!("==", "\\equiv"),
			DefNotEquals => normal_or_md!("!=", "\\not\\equiv"),
			RoughlyEquals => normal_or_md!("~=", "\\approx"),
			Greater => ">",
			GreaterEqual => normal_or_md!(">=", "\\geq"),
			Lesser => "<",
			LesserEqual => normal_or_md!("<=", "\\leq"),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_divisible(&mut self, node: &TheoryNode, expr: &TheoryNode, divisor: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Divisible => normal_or_md!("%", "\\operatorname{\\%}"),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(expr), this, self.visit(divisor))
	}

	fn visit_exists(&mut self, node: &TheoryNode, expr: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Exists => normal_or_md!("??", "\\operatorname{??}"),
			_ => unreachable!(),
		};

		format!("{} {}", self.visit(expr), this)
	}

	fn visit_grouping(&mut self, _node: &TheoryNode, expr: &TheoryNode) -> String {
		normal_or_md!(
			format!("({})", self.visit(expr)),
			format!("\\left( {} \\right)", self.visit(expr))
		)
	}

	fn visit_expression(&mut self, _node: &TheoryNode, expr: &ExprNode) -> String {
		//
		format!("{}", ExprPrinter::print(expr))
	}
}

impl TheoryPrinter {
	pub fn print(root: &TheoryNode) -> String {
		TheoryPrinter{}.visit(root)
	}
}

// ================= Expr =================

pub struct ExprPrinter;

impl ExprVisitor<String> for ExprPrinter {
	fn visit_equality(&mut self, node: &ExprNode, lhs: &ExprNode, rhs: &ExprNode) -> String {
		let this = match node.token.kind {
			Equals => "=",
			NotEquals => normal_or_md!("/=", "\\neq"),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_term(&mut self, node: &ExprNode, lhs: &ExprNode, rhs: &ExprNode) -> String {
		let this = match node.token.kind {
			Plus => "+",
			Minus => "-",
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_factor(&mut self, node: &ExprNode, lhs: &ExprNode, rhs: &ExprNode) -> String {
		let this = match node.token.kind {
			Multiply => "*",
			Divide => normal_or_md!("/", {
				return format!("\\frac{{{}}}{{{}}}", self.visit(lhs), self.visit(rhs))
			}),
			_ => unreachable!(),
		};

		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_unary(&mut self, node: &ExprNode, expr: &ExprNode) -> String {
		let this = match node.token.kind {
			Minus => "-",
			_ => unreachable!(),
		};

		format!("{}{}", this, self.visit(expr))
	}

	fn visit_power(&mut self, _node: &ExprNode, base: &ExprNode, power: &ExprNode) -> String {
		format!("{}^{}", self.visit(base), self.visit(power))
	}

	fn visit_grouping(&mut self, _node: &ExprNode, expr: &ExprNode) -> String {
		normal_or_md!(
			format!("({})", self.visit(expr)),
			format!("\\left( {} \\right)", self.visit(expr))
		)
	}

	fn visit_literal(&mut self, _node: &ExprNode, literal: &Literal) -> String {
		match literal {
			Literal::Integer(v) => v.to_string(),
			Literal::Float(v) => v.to_string(),
		}
	}

	fn visit_variable(&mut self, _node: &ExprNode, path: &String, _expr: &ExprNode) -> String {
		//
		normal_or_md!(path.to_string(), format!("\\text{{{}}}", path))
	}
}

impl ExprPrinter {
	pub fn print(root: &ExprNode) -> String {
		ExprPrinter{}.visit(root)
	}
}
