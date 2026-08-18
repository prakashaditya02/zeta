use crate::token::*;
use crate::expr::*;
use crate::token::TokenType as tt;
use crate::precedence::Precedence as Prec;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let c = &self.tokens[self.current];
        // if c.token != TokenType::Eof {
        //     self.current += 1;
        // }
        match c.token {
            TokenType::Number => return Expr::Number(c.lexeme.parse().unwrap()),
            _ => return Expr::Nil
        }
    }

    fn get_binary_op(&self, token_type: &tt) -> BinaryOp {
        match token_type {
            tt::Plus => BinaryOp::Add,
            tt::Minus => BinaryOp::Subtract,
            tt::Star => BinaryOp::Multiply,
            tt::Slash => BinaryOp::Divide,
            // tt::Equal => BinaryOp::Equal,
            // tt::BangEqual => BinaryOp::NotEqual,
            // tt::Greater => BinaryOp::Greater,
            // tt::GreaterEqual => BinaryOp::GreaterEqual,
            // tt::Less => BinaryOp::Less,
            // tt::LessEqual => BinaryOp::LessEqual,
            _ => panic!()
        }
    }

    fn get_infix_prec(&self, token_type: &tt) -> Prec {
        match token_type {
            tt::Plus | tt::Minus => Prec::Term,
            tt::Star | tt::Slash => Prec::Factor,
            tt::BangEqual | tt::EqualEqual => Prec::Equality,
            tt::Less | tt::Greater | tt::LessEqual | tt::GreaterEqual => Prec::Comparison,
            tt::Or => Prec::Or,
            tt::And => Prec::And,
            tt::Equal => Prec::Assignment,
            _ => Prec::None 
        }
    }

    fn parse_prec(&mut self, min_prec: Prec) -> Expr {
        let mut left = self.parse_expr();
        self.advance();

        let mut prev = self.get_infix_prec(&self.peek().token);
        while prev > min_prec {
            let operator = self.get_binary_op(&self.peek().token);
            self.advance();
            let right = self.parse_prec(prev);
            left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
            prev = self.get_infix_prec(&self.peek().token);
        }

        return left;
    }

    fn advance(&mut self) {
        if (self.tokens[self.current]).token != tt::Eof {
            self.current += 1;
        }
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Expr {
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan_tokens();

        let mut parser = Parser::new(tokens);
        parser.parse_prec(Prec::None)
    }

    #[test]
    fn parse_number() {
        let expr = parse("123");
        assert_eq!(expr, Expr::Number(123.0));
    }

    #[test]
    fn parse_add() {
        let expr = parse("1 + 2");
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Number(1.0)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0))
        });
    }

        #[test]
    fn parses_multiplication() {
        let expr = parse("2 * 3");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                operator: BinaryOp::Multiply,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

        #[test]
    fn parses_precedence() {
        let expr = parse("1 + 2 * 3");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(2.0)),
                        operator: BinaryOp::Multiply,
                        right: Box::new(Expr::Number(3.0)),
                    }
                ),
            }
        );
    }
}