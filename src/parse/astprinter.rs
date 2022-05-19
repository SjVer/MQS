use super::ast::{
	*,
	TheoryItem::*,
	ExprItem::*,
	Literal as ALiteral
};
use crate::lex::token::TokenKind::{self, *};

// ================ Theory ================

pub struct TheoryPrinter;

impl TheoryVisitor<String> for TheoryPrinter {
	fn visit_logical(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			Or => "|",
			XOr => "!|",
			And => "&",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Logical { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_match(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			Matches => "<>",
			NotMatches => "<!",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Match { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_comparison(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			DefEquals => "==",
			DefNotEquals => "!=",
			Greater => ">",
			GreaterEqual => ">=",
			Lesser => "<",
			LesserEqual => "<=",
			RoughlyEquals => "~",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Comparison { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_divisible(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Divisible => "%",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let TheoryItem::Divisible { expr, divisor } = &node.item {
			(self.visit(&expr), self.visit(&divisor))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_exists(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Exists => "??",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Logical { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this)
	}

}

impl TheoryPrinter {
	pub fn print(root: &TheoryNode) {
		println!("{}", TheoryPrinter{}.visit(root));
	}
}

// ================= Expr =================

pub struct ExprPrinter;

impl ExprVisitor<String> for ExprPrinter {
	fn visit_equality(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Equals => "=",
			NotEquals => "/=",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Equality { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_term(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Plus => "+",
			Minus => "-",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Term { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_factor(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Multiply => "*",
			Divide => "/",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if let Factor { lhs, rhs } = &node.item {
			(self.visit(&lhs), self.visit(&rhs))
		} else { unreachable!() };

		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_unary(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Minus => "-",
			_ => unreachable!(),
		};

		let expr = if let Unary(expr) = &node.item {
			self.visit(&expr)
		} else { unreachable!() };

		format!("{}{}", this, expr)
	}

	fn visit_power(&mut self, node: &ExprNode) -> String {
		let (base, power) = 
			if let ExprItem::Power { base, power } = &node.item {
				(self.visit(&base), self.visit(&power))
			} else { unreachable!() };

		format!("{}^{}", base, power)
	}

	fn visit_grouping(&mut self, node: &ExprNode) -> String {
		let expr = if let ExprItem::Grouping(expr) = &node.item {
			self.visit(&expr)
		} else { unreachable!() };

		format!("({})", expr)
	}

	fn visit_literal(&mut self, node: &ExprNode) -> String {
		if let Literal(literal) = &node.item {
			match literal {
				ALiteral::Integer(v) => v.to_string(),
				ALiteral::Float(v) => v.to_string(),
			}
		} else { unreachable!() }
	}
}

impl ExprPrinter {
	pub fn print(root: &ExprNode) {
		println!("{}", ExprPrinter{}.visit(root));
	}
}
