#[derive(PartialEq, Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Or, True, False, Fun, Let, For, While, If, Else, Print, Return, Nil, Struct, Enum,
    Eof
}

pub struct Token {
    pub lexeme: String,
    pub token: TokenType,
    pub line: usize,
}
