#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Pipe,
    Term,
    Factor,
    Unary,
    Call,
    Primary
}