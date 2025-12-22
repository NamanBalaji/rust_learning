use crate::tokenize::Token;

pub struct AssignmentImpl {
    pub target: VariableImpl,
    pub value: Box<Expr>,
}

pub struct BinaryOpImpl {
    pub lhs: Box<Expr>,
    pub operation: Token,
    pub rhs: Box<Expr>,
}

pub struct FunCallImpl {
    pub name: VariableImpl,
    pub arg: Box<Expr>,
}

pub struct NumberImpl {
    pub value: i32,
}

pub struct VariableImpl {
    pub name: Token,
}

pub enum Expr {
    Assignment(AssignmentImpl),
    BinaryOperation(BinaryOpImpl),
    FunCall(FunCallImpl),
    Number(NumberImpl),
    Variable(VariableImpl),
}
