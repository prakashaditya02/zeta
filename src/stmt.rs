use crate::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Stmt{
    Expression(Expr),
    Let,
}