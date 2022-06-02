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
			Or => "|",
			And => "&",
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node Logical => lhs rhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_unary(&mut self, node: &TheoryNode, expr: &TheoryNode) -> String {
		let this = match node.token.kind {
			Not => "~",
			_ => unreachable!(),
		};

		// let expr = if_let_single!(self node TheoryItem::Unary);
		format!("{} {}", this, self.visit(expr))
	}

	fn visit_implies(&mut self, node: &TheoryNode, lhs: &TheoryNode, rhs: &TheoryNode) -> String {
		let this = match node.token.kind {
			Implies => normal_or_md!("<=>", "\\iff"),
			NotImplies => normal_or_md!("<!>", "\\not\\iff"),
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node Match => rhs lhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_comparison(&mut self, node: &TheoryNode, lhs: &TheoryNode, rhs: &TheoryNode) -> String {
		let this = match node.token.kind {
			DefEquals => "==",
			DefNotEquals => "!=",
			RoughlyEquals => "~=",
			Greater => ">",
			GreaterEqual => ">=",
			Lesser => "<",
			LesserEqual => "<=",
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node Comparison => rhs lhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_divisible(&mut self, node: &TheoryNode, expr: &TheoryNode, divisor: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Divisible => "%",
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node TheoryItem::Divisible => expr divisor);
		format!("{} {} {}", self.visit(expr), this, self.visit(divisor))
	}

	fn visit_exists(&mut self, node: &TheoryNode, expr: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Exists => "??",
			_ => unreachable!(),
		};

		// let expr = if_let_single!(self node TheoryItem::Exists);
		format!("{} {}", self.visit(expr), this)
	}

	fn visit_grouping(&mut self, _node: &TheoryNode, expr: &TheoryNode) -> String {
		// let th = if_let_single!(self node TheoryItem::Grouping);
		format!("({})", self.visit(expr))
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

		// let (lhs, rhs) = if_let_binary!(self node Equality => lhs rhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_term(&mut self, node: &ExprNode, lhs: &ExprNode, rhs: &ExprNode) -> String {
		let this = match node.token.kind {
			Plus => "+",
			Minus => "-",
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node Term => lhs rhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_factor(&mut self, node: &ExprNode, lhs: &ExprNode, rhs: &ExprNode) -> String {
		let this = match node.token.kind {
			Multiply => "*",
			Divide => "/",
			_ => unreachable!(),
		};

		// let (lhs, rhs) = if_let_binary!(self node Factor => lhs rhs);
		format!("{} {} {}", self.visit(lhs), this, self.visit(rhs))
	}

	fn visit_unary(&mut self, node: &ExprNode, expr: &ExprNode) -> String {
		let this = match node.token.kind {
			Minus => "-",
			_ => unreachable!(),
		};

		// let expr = if_let_single!(self node ExprItem::Unary);
		format!("{}{}", this, self.visit(expr))
	}

	fn visit_power(&mut self, _node: &ExprNode, base: &ExprNode, power: &ExprNode) -> String {
		// let (base, power) = if_let_binary!(self node ExprItem::Power => base power);
		format!("{}^{}", self.visit(base), self.visit(power))
	}

	fn visit_grouping(&mut self, _node: &ExprNode, expr: &ExprNode) -> String {
		// let expr = if_let_single!(self node ExprItem::Grouping);
		format!("({})", self.visit(expr))
	}

	fn visit_literal(&mut self, _node: &ExprNode, literal: &Literal) -> String {
		match literal {
			Literal::Integer(v) => v.to_string(),
			Literal::Float(v) => v.to_string(),
		}
	}

	fn visit_variable(&mut self, _node: &ExprNode, path: &String, _expr: &ExprNode) -> String {
		//
		path.to_string()
	}
}

impl ExprPrinter {
	pub fn print(root: &ExprNode) -> String {
		ExprPrinter{}.visit(root)
	}
}
