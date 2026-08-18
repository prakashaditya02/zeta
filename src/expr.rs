#[derive(PartialEq, Debug)]
pub enum Expr {
    Binary{ left: Box<Expr>, operator: BinaryOp, right: Box<Expr>},
    Unary{ operator: UnaryOp, right: Box<Expr>},
    Number(f64),
    Grouping(Box<Expr>),
    String(String),
    Bool(bool),
    Nil
}

#[derive(PartialEq, Debug)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide,
    Equal, NotEqual, Greater, GreaterEqual, Less, LessEqual,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Negate, Not, 
}