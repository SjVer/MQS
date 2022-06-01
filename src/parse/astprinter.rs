use super::ast::{
	*,
	TheoryItem::*,
	ExprItem::*,
	Literal as ALiteral
};
use crate::lex::token::TokenKind::{self, *};

// ================ Theory ================

pub struct TheoryPrinter;

macro_rules! if_let_binary {
	($self:ident $node:ident $what:path => $a:ident $b:ident) => {
		if let $what { $a, $b } = &$node.item {
			($self.visit(&$a), $self.visit(&$b))
		} else {
			unreachable!()
		}
	};
}
macro_rules! if_let_single {
	($self:ident $node:ident $what:path) => {
		if let $what ( a ) = &$node.item {
			$self.visit(&a)
		} else {
			unreachable!()
		}
	};
}

impl TheoryVisitor<String> for TheoryPrinter {
	fn visit_logical(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			Or => "|",
			And => "&",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node Logical => lhs rhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_unary(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			Not => "~",
			_ => unreachable!(),
		};

		let expr = if_let_single!(self node TheoryItem::Unary);
		format!("{} {}", this, expr)
	}

	fn visit_match(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			Implies => "<=>",
			NotImplies => "<!>",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node Match => rhs lhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_comparison(&mut self, node: &TheoryNode) -> String {
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

		let (lhs, rhs) = if_let_binary!(self node Comparison => rhs lhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_divisible(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Divisible => "%",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node TheoryItem::Divisible => expr divisor);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_exists(&mut self, node: &TheoryNode) -> String {
		let this = match node.token.kind {
			TokenKind::Exists => "??",
			_ => unreachable!(),
		};

		let expr = if_let_single!(self node TheoryItem::Exists);
		format!("{} {}", expr, this)
	}

	fn visit_grouping(&mut self, node: &TheoryNode) -> String {
		let th = if_let_single!(self node TheoryItem::Grouping);
		format!("({})", th)
	}

	fn visit_expression(&mut self, node: &TheoryNode) -> String {
		if let TheoryItem::Expression ( expr ) = &node.item {
			format!("{}", ExprPrinter::print(&expr))
		} else {
			unreachable!()
		}
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
	fn visit_equality(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Equals => "=",
			NotEquals => "/=",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node Equality => lhs rhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_term(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Plus => "+",
			Minus => "-",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node Term => lhs rhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_factor(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Multiply => "*",
			Divide => "/",
			_ => unreachable!(),
		};

		let (lhs, rhs) = if_let_binary!(self node Factor => lhs rhs);
		format!("{} {} {}", lhs, this, rhs)
	}

	fn visit_unary(&mut self, node: &ExprNode) -> String {
		let this = match node.token.kind {
			Minus => "-",
			_ => unreachable!(),
		};

		let expr = if_let_single!(self node ExprItem::Unary);
		format!("{}{}", this, expr)
	}

	fn visit_power(&mut self, node: &ExprNode) -> String {
		let (base, power) = if_let_binary!(self node ExprItem::Power => base power);
		format!("{}^{}", base, power)
	}

	fn visit_grouping(&mut self, node: &ExprNode) -> String {
		let expr = if_let_single!(self node ExprItem::Grouping);
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

	fn visit_variable(&mut self, node: &ExprNode) -> String {
		if let ExprItem::Variable{ path, .. } = &node.item { path.to_string() }
		else { unreachable!() }
	}
}

impl ExprPrinter {
	pub fn print(root: &ExprNode) -> String {
		ExprPrinter{}.visit(root)
	}
}
