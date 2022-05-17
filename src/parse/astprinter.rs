use super::ast::{ASTItem::*, *, Literal as ALiteral};
use crate::lex::token::TokenKind::*;

pub struct ASTPrinter;

impl ASTVisitor<String> for ASTPrinter {
	fn visit_equality(&mut self, node: &ASTNode) -> String {
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

	fn visit_term(&mut self, node: &ASTNode) -> String {
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

	fn visit_factor(&mut self, node: &ASTNode) -> String {
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

	fn visit_unary(&mut self, node: &ASTNode) -> String {
		let this = match node.token.kind {
			Minus => "-",
			_ => unreachable!(),
		};

		let expr = if let Unary(expr) = &node.item {
			self.visit(&expr)
		} else { unreachable!() };

		format!("{}{}", this, expr)
	}

	fn visit_power(&mut self, node: &ASTNode) -> String {
		let (base, power) = 
			if let ASTItem::Power { base, power } = &node.item {
				(self.visit(&base), self.visit(&power))
			} else { unreachable!() };

		format!("{}^{}", base, power)
	}

	fn visit_grouping(&mut self, node: &ASTNode) -> String {
		let expr = if let Grouping(expr) = &node.item {
			self.visit(&expr)
		} else { unreachable!() };

		format!("({})", expr)
	}

	fn visit_literal(&mut self, node: &ASTNode) -> String {
		if let Literal(literal) = &node.item {
			match literal {
				ALiteral::Integer(v) => v.to_string(),
				ALiteral::Float(v) => v.to_string(),
			}
		} else { unreachable!() }
	}
}

impl ASTPrinter {
	pub fn print(root: &ASTNode) {
		println!("{}", ASTPrinter{}.visit(root));
	}
}