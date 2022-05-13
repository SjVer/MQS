use crate::lex::token::Token;

pub trait ASTVisitor {
	fn visit(&self, node: &ASTNode);
}

macro_rules! __node {
	($name:ident (, $field:tt)*) => {
		$name { token: Token $(, $field)* }
	};
}

pub enum ASTNode {
	BinaryExpr { tok: Token, lhs: Box<ASTNode>, rhs: Box<ASTNode> },
	Literal { tok: Token },
}