use crate::lex::token::Token;

pub struct ASTNode {
	pub token: Token,
	pub item: ASTItem,
}

pub enum ASTItem {
	Equality	{ lhs: Box<ASTNode>, rhs: Box<ASTNode> },
	Term 		{ lhs: Box<ASTNode>, rhs: Box<ASTNode> },
	Factor 		{ lhs: Box<ASTNode>, rhs: Box<ASTNode> },
	Unary		(Box<ASTNode>),
	Power 		{ base: Box<ASTNode>, power: Box<ASTNode> },
	Grouping	(Box<ASTNode>),
	Literal		(Literal),
}

pub enum Literal {
	Integer(u64),
	Float(f64),
}

pub trait ASTVisitor<T> {
    fn visit(&mut self, node: &ASTNode) -> T {
        match node.item {
            ASTItem::Equality	{..} => self.visit_equality(node),
            ASTItem::Term		{..} => self.visit_term(node),
            ASTItem::Factor		{..} => self.visit_factor(node),
            ASTItem::Unary		(..) => self.visit_unary(node),
            ASTItem::Power		{..} => self.visit_power(node),
            ASTItem::Grouping	{..} => self.visit_grouping(node),
            ASTItem::Literal	{..} => self.visit_literal(node),
        }
    }
	
    fn visit_equality(&mut self, node: &ASTNode) -> T;
    fn visit_term(&mut self, node: &ASTNode) -> T;
    fn visit_factor(&mut self, node: &ASTNode) -> T;
    fn visit_unary(&mut self, node: &ASTNode) -> T;
    fn visit_power(&mut self, node: &ASTNode) -> T;
    fn visit_grouping(&mut self, node: &ASTNode) -> T;
    fn visit_literal(&mut self, node: &ASTNode) -> T;
}