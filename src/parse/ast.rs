use crate::lex::token::Token;

pub trait ASTVisitor {
	fn visit(&self, node: &ASTNode);
}

pub enum ASTItem {
	BinaryExpr { lhs: Box<ASTNode>, rhs: Box<ASTNode> },
	Literal {},
}

pub struct ASTNode {
	token: Token,
	item: ASTItem,
}