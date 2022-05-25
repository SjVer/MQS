use crate::lex::token::Token;

// ================ Theory ================

#[derive(Clone, Debug)]
pub struct TheoryNode {
	pub token: Token,
	pub item: TheoryItem
}

#[derive(Clone, Debug)]
pub enum TheoryItem {
	Logical     { lhs: Box<TheoryNode>, rhs: Box<TheoryNode> },
	Match       { lhs: Box<TheoryNode>, rhs: Box<TheoryNode> },
	Comparison  { lhs: Box<TheoryNode>, rhs: Box<TheoryNode> },
	Divisible	{ expr: Box<TheoryNode>, divisor: Box<TheoryNode> },
	Exists  	(Box<TheoryNode>),
	Grouping  	(Box<TheoryNode>),
	Expression 	(ExprNode),
}

pub trait TheoryVisitor<T> {
	fn visit(&mut self, node: &TheoryNode) -> T {
		match node.item {
			TheoryItem::Logical		{..} => self.visit_logical(node),
			TheoryItem::Match		{..} => self.visit_match(node),
			TheoryItem::Comparison	{..} => self.visit_comparison(node),
			TheoryItem::Divisible	{..} => self.visit_divisible(node),
			TheoryItem::Exists		(..) => self.visit_exists(node),
			TheoryItem::Grouping	(..) => self.visit_grouping(node),
			TheoryItem::Expression	(..) => self.visit_expression(node),
		}
	}
	
	fn visit_logical(&mut self, node: &TheoryNode) -> T;
	fn visit_match(&mut self, node: &TheoryNode) -> T;
	fn visit_comparison(&mut self, node: &TheoryNode) -> T;
	fn visit_divisible(&mut self, node: &TheoryNode) -> T;
	fn visit_exists(&mut self, node: &TheoryNode) -> T;
	fn visit_grouping(&mut self, node: &TheoryNode) -> T;
	fn visit_expression(&mut self, node: &TheoryNode) -> T;
}

// ================= Expr =================

#[derive(Clone, Debug)]
pub struct ExprNode {
	pub token: Token,
	pub item: ExprItem,
}

#[derive(Clone, Debug)]
pub enum ExprItem {
	Equality	{ lhs: Box<ExprNode>, rhs: Box<ExprNode> },
	Term 		{ lhs: Box<ExprNode>, rhs: Box<ExprNode> },
	Factor 		{ lhs: Box<ExprNode>, rhs: Box<ExprNode> },
	Unary		(Box<ExprNode>),
	Power 		{ base: Box<ExprNode>, power: Box<ExprNode> },
	Grouping	(Box<ExprNode>),
	Variable	{path: String, expr: Box<ExprNode>},
	Literal		(Literal),
}

#[derive(Clone, Debug)]
pub enum Literal {
	Integer(u64),
	Float(f64),
}

pub trait ExprVisitor<T> {
	fn visit(&mut self, node: &ExprNode) -> T {
		match node.item {
			ExprItem::Equality	{..} => self.visit_equality(node),
			ExprItem::Term		{..} => self.visit_term(node),
			ExprItem::Factor	{..} => self.visit_factor(node),
			ExprItem::Unary		(..) => self.visit_unary(node),
			ExprItem::Power		{..} => self.visit_power(node),
			ExprItem::Grouping	{..} => self.visit_grouping(node),
			ExprItem::Variable	{..} => self.visit_variable(node),
			ExprItem::Literal	{..} => self.visit_literal(node),
		}
	}
	
	fn visit_equality(&mut self, node: &ExprNode) -> T;
	fn visit_term(&mut self, node: &ExprNode) -> T;
	fn visit_factor(&mut self, node: &ExprNode) -> T;
	fn visit_unary(&mut self, node: &ExprNode) -> T;
	fn visit_power(&mut self, node: &ExprNode) -> T;
	fn visit_grouping(&mut self, node: &ExprNode) -> T;
	fn visit_variable(&mut self, node: &ExprNode) -> T;
	fn visit_literal(&mut self, node: &ExprNode) -> T;
}