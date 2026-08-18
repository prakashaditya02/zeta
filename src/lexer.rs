use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;
use std::string::String;
pub struct Lexer {
    source: String,
    line: usize,
    start: usize,
    current: usize
}

impl Lexer {
    pub fn new(source: String) -> Self {
        if !source.is_ascii() {
            panic!("Source should not contain non-ASCII characters");
        }
        Lexer {
            source,
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }
        tokens.push(Token{
            lexeme: String::new(),
            token: Eof,
            line: self.line
        });
        return tokens;
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = &self.source[self.current..self.current+1];
        match c {
            "(" => { self.current+=1; return Some(self.build_token(LeftParen))}
            ")" => { self.current+=1; return Some(self.build_token(RightParen))}
            "{" => { self.current+=1; return Some(self.build_token(LeftBrace)) }
            "}" => { self.current+=1; return Some(self.build_token(RightBrace));}
            "," => { self.current+=1; return Some(self.build_token(Comma))}
            "." => { self.current+=1; return Some(self.build_token(Dot))}
            "-" => { self.current+=1; return Some(self.build_token(Minus))}
            "+" => { self.current+=1; return Some(self.build_token(Plus))}
            ";" => { self.current+=1; return Some(self.build_token(Semicolon))}
            "*" => { self.current+=1; return Some(self.build_token(Star))}

            "!" => {if self.peek_next("="){
                self.current += 2;
                return Some(self.build_token(BangEqual))
            } else {
                self.current += 1;
                return Some(self.build_token(Bang))}
            }

            "=" => {if self.peek_next("="){
                self.current += 2;
                return Some(self.build_token(EqualEqual))
            } else {
                self.current += 1;
                return Some(self.build_token(Equal))
            }}

            ">" => {if self.peek_next("="){
                self.current += 2;
                return Some(self.build_token(GreaterEqual))
            } else {
                self.current += 1;
                return Some(self.build_token(Greater))
            }}

            "<" => {if self.peek_next("="){
                self.current += 2;
                return Some(self.build_token(LessEqual))
            } else {
                self.current += 1;
                return Some(self.build_token(Less))
            }}

            "\r" | "\t" | " " => {
                self.current += 1;
                return None
            },

            "\n" => {
                self.line += 1;
                self.current += 1;
                return None;
            },

            "\"" => {
                self.start += 1;
                if self.string_lex() {
                    let token = self.build_token(TokenType::String);
                    self.current+=1;
                    return Some(token);
                } else {
                    //panic!("Unterminated string");
                    return None
                }
            },

            "/" => {
                if self.peek_next("/"){
                    self.current += 2;
                    while !self.is_at_end() && self.peek() != "\n" {
                        self.current += 1;
                    }
                    return None
                } else {
                    self.current += 1;
                    return Some(self.build_token(Slash))
                }
            }
            _ => if is_digit(c) {
                self.current += 1;
                self.number();
                Some(self.build_token(Number))
            } else if is_alphabet(c) || (c == "_") {
                self.current += 1;
                self.identifier();
                let ttype: TokenType = check_keyword(&self.source[self.start..self.current]); 
                Some(self.build_token(ttype))
            } else {
                return None
            }
        }
    }

    fn is_at_end(&self) -> bool {
        return self.source.len() <= self.current;
    }

    fn has_next(&self) -> bool {
        self.current + 1 < self.source.len()
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            return "\0"
        }
        &self.source[self.current..self.current+1]
    }

    fn peek_next(&self, expected: &str) -> bool {
        if self.current+1 >= self.source.len() { 
            return false;
        }
        &self.source[self.current+1..self.current+2] == expected
    }

    fn string_lex(&mut self) -> bool {
        while self.has_next() && !self.peek_next("\""){
            if self.peek_next("\n") {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.has_next() && self.peek_next("\"") {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn build_token(&self, ty: TokenType) -> Token {
        let s = &self.source[self.start..self.current];
        let tok = Token{lexeme: s.to_string(), token: ty, line: self.line};
        return tok;
    }

    fn number(&mut self) {
        while !self.is_at_end() && is_digit(self.peek()) {
            self.current += 1;
        }

        if !self.is_at_end() && self.peek() == "." {
            self.current += 1;
            while !self.is_at_end() && is_digit(self.peek()) {
                self.current += 1;
            }
        }
    }

    fn identifier(&mut self) {
        while !self.is_at_end() && (self.peek() == "_" || is_alphanum(self.peek())) {
            self.current += 1;
        }
    }

}

fn is_digit(c: &str) -> bool {
        return c.chars().all(|c| c.is_ascii_digit())
}

fn is_alphabet(c: &str) -> bool {
        return c.chars().all(|c| c.is_ascii_alphabetic())
}

fn is_alphanum(c: &str) -> bool {
    return is_digit(c) || is_alphabet(c)
}

fn check_keyword(lex: &str) -> TokenType {
    match lex {
        "and" => And,
        "or" => Or,
        "true" => True,
        "false" => False,
        "fun" => Fun,
        "let" => Let,
        "for" => For,
        "while" => While,
        "if" => If,
        "else" => Else,
        "print" => Print,
        "return" => Return,
        "nil" => Nil,
        "struct" => Struct,
        "enum" => Enum,
        _ => Identifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn lexer_checker() {
        let mut lexer = Lexer::new("let 123.45 () -+//Hi".to_string());
        let tokens = lexer.scan_tokens();

        assert_eq!(tokens.len(), 7);

        assert_eq!(tokens[0].lexeme, "let");
        assert_eq!(tokens[0].token, Let);

        assert_eq!(tokens[1].lexeme, "123.45");
        assert_eq!(tokens[1].token, Number);

        assert_eq!(tokens[2].lexeme, "(");
        assert_eq!(tokens[2].token, LeftParen);

        assert_eq!(tokens[3].lexeme, ")");
        assert_eq!(tokens[3].token, RightParen);

        assert_eq!(tokens[4].lexeme, "-");
        assert_eq!(tokens[4].token, Minus);

        assert_eq!(tokens[5].lexeme, "+");
        assert_eq!(tokens[5].token, Plus);

        assert_eq!(tokens[6].token, Eof);
        assert_eq!(tokens[6].lexeme, "");
    }

    #[test]
    fn lexer_string() {
        let mut lexer = Lexer::new("\"hello world\"".to_string());
        let tokens = lexer.scan_tokens();

        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].token, String);
        assert_eq!(tokens[0].lexeme, "hello world");
        
        assert_eq!(tokens[1].token, Eof);
    }
    
}

