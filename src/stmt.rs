use crate::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Stmt{
    Expression(Expr),
    Let {name: String, value: Option<Expr>},
    Block(Vec<Stmt>),
    If {condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>>},
    While {condition: Expr, body: Vec<Stmt>},
    //For {init: Box<Stmt>, condition: Expr, increment: Expr, body: Vec<Stmt>},
    Fun {name: String, parameters: Vec<String>, body: Vec<Stmt>},
    Return(Option<Expr>),
    //Struct
    //Enum
}