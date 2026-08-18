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
        match c.token {

            TokenType::Number => {
                let num = c.lexeme.parse().unwrap();
                self.advance();
                return Expr::Number(num)
            }

            TokenType::Minus | TokenType::Bang => {
                let operator = self.get_unary_op(&self.peek().token);
                self.advance();
                let right = self.parse_prec(Prec::Unary);
                return Expr::Unary { 
                    operator: operator, 
                    right: Box::new(right)
                }
            },

            TokenType::LeftParen => {
                self.advance();
                let group = self.parse_prec(Prec::None);
                if self.peek().token != TokenType::RightParen {
                    panic!();
                }
                self.advance();
                return Expr::Grouping(Box::new(group));
            },

            TokenType::String => {
                let val = c.lexeme.clone();
                self.advance();
                Expr::String(val)
            },

            TokenType::Identifier => {
                let var = c.lexeme.clone();
                self.advance();
                Expr::Identifier(var)
            }

            TokenType::True => {
                self.advance();
                Expr::Bool(true)
            },

            TokenType::False => {
                self.advance();
                Expr::Bool(false)
            },

            TokenType::Nil => {
                self.advance();
                Expr::Nil
            }

            _ => panic!()
        }
    }

    fn get_unary_op(&self, token_type: &tt) -> UnaryOp {
        match token_type {
            tt::Bang => UnaryOp::Not,
            tt::Minus => UnaryOp::Negate,
            _ => panic!()
        }
    }

    fn get_binary_op(&self, token_type: &tt) -> BinaryOp {
        match token_type {
            tt::Plus => BinaryOp::Add,
            tt::Minus => BinaryOp::Subtract,
            tt::Star => BinaryOp::Multiply,
            tt::Slash => BinaryOp::Divide,
            tt::EqualEqual => BinaryOp::Equal,
            tt::BangEqual => BinaryOp::NotEqual,
            tt::Greater => BinaryOp::Greater,
            tt::GreaterEqual => BinaryOp::GreaterEqual,
            tt::Less => BinaryOp::Less,
            tt::LessEqual => BinaryOp::LessEqual,
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

        let mut precedence = self.get_infix_prec(&self.peek().token);
        while precedence > min_prec {
            let operator = self.get_binary_op(&self.peek().token);

            self.advance();
            let right = self.parse_prec(precedence);
            left = Expr::Binary { 
                left: Box::new(left),
                operator: operator,
                right: Box::new(right) };

            precedence = self.get_infix_prec(&self.peek().token);
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

    #[test]
    fn parses_negation() {
        let expr = parse("-123");

        assert_eq!(
            expr,
            Expr::Unary {
                operator: UnaryOp::Negate,
                right: Box::new(Expr::Number(123.0)),
            }
        );
    }

    #[test]
    fn parses_double_negation() {
        let expr = parse("--123");

        assert_eq!(
            expr,
            Expr::Unary {
                operator: UnaryOp::Negate,
                right: Box::new(
                    Expr::Unary {
                        operator: UnaryOp::Negate,
                        right: Box::new(Expr::Number(123.0)),
                    }
                ),
            }
        );
    }

    #[test]
    fn grouping_overrides_precedence() {
        let expr = parse("(1 + 2) * 3");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(
                    Expr::Grouping(
                        Box::new(
                            Expr::Binary {
                                left: Box::new(Expr::Number(1.0)),
                                operator: BinaryOp::Add,
                                right: Box::new(Expr::Number(2.0)),
                            }
                        )
                    )
                ),
                operator: BinaryOp::Multiply,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn parses_nested_grouping() {
        let expr = parse("((123))");

        assert_eq!(
            expr,
            Expr::Grouping(
                Box::new(
                    Expr::Grouping(
                        Box::new(Expr::Number(123.0))
                    )
                )
            )
        );
    }

    #[test]
    fn parses_unary_with_binary() {
        let expr = parse("-1 + 2");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(
                    Expr::Unary {
                        operator: UnaryOp::Negate,
                        right: Box::new(Expr::Number(1.0)),
                    }
                ),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn parses_identifier_expression() {
        let expr = parse("foo + 123 * bar");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(
                    Expr::Identifier("foo".to_string())
                ),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(123.0)),
                        operator: BinaryOp::Multiply,
                        right: Box::new(
                            Expr::Identifier("bar".to_string())
                        ),
                    }
                ),
            }
        );
    }

}