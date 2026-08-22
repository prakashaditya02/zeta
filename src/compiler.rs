use crate::chunk::*;
use crate::expr::*;
use crate::value::*;
use crate::opcode::*;

pub struct Compiler {
    pub chunk: Chunk,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler{
            chunk: Chunk::new(),
        }
    }

    fn compile(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(val) => {
                let index = self.chunk.add_constant(Value::Number(*val));
                self.chunk.write_opcode(OpCode::Constant, 0);
                self.chunk.write(index.try_into().unwrap(), 0);
            },

            Expr::Nil => {
                self.chunk.write_opcode(OpCode::Nil, 0);
            },

            Expr::Bool(val) => {
                if *val == true { 
                    self.chunk.write_opcode(OpCode::True, 0)
                } else { 
                    self.chunk.write_opcode(OpCode::False, 0) 
                }
            },

            Expr::Unary { operator, right } => {
                self.compile(&right);

                match operator {
                    UnaryOp::Negate => self.chunk.write_opcode(OpCode::Negate, 0),
                    UnaryOp::Not => self.chunk.write_opcode(OpCode::Not, 0),
                }
            },

            Expr::Binary { left, operator, right } => {
                self.compile(&left);
                self.compile(&right);

                match operator {
                    BinaryOp::Add => self.chunk.write_opcode(OpCode::Add, 0),
                    BinaryOp::Subtract => self.chunk.write_opcode(OpCode::Subtract, 0),
                    BinaryOp::Multiply => self.chunk.write_opcode(OpCode::Multiply, 0),
                    BinaryOp::Divide => self.chunk.write_opcode(OpCode::Divide, 0),

                    BinaryOp::Equal => self.chunk.write_opcode(OpCode::Equal, 0),
                    BinaryOp::NotEqual => {
                        self.chunk.write_opcode(OpCode::Equal, 0);
                        self.chunk.write_opcode(OpCode::Not, 0);
                    },

                    BinaryOp::Greater => self.chunk.write_opcode(OpCode::Greater, 0),
                    BinaryOp::GreaterEqual => {
                        self.chunk.write_opcode(OpCode::Less, 0);
                        self.chunk.write_opcode(OpCode::Not, 0);
                    },

                    BinaryOp::Less => self.chunk.write_opcode(OpCode::Less, 0),
                    BinaryOp::LessEqual => {
                        self.chunk.write_opcode(OpCode::Greater, 0);
                        self.chunk.write_opcode(OpCode::Not, 0);
                    }
                }
            },

            Expr::Grouping(val) => {
                self.compile(&val);
            },

            Expr::String(str) => {
                let index = self.chunk.add_constant(Value::String(str.to_string()));
                self.chunk.write_opcode(OpCode::Constant, 0);
                self.chunk.write(index.try_into().unwrap(), 0);
            },

            _ => {panic!()}
        }
    }
}

#[test]
fn compiles_number_literal() {
    let mut compiler = Compiler::new();
    compiler.compile(&Expr::Number(5.0));

    assert_eq!(
        compiler.chunk.code,
        vec![
            OpCode::Constant as u8,
            0,
        ]
    );

    assert_eq!(
        compiler.chunk.constants,
        vec![
            Value::Number(5.0)
        ]
    );
}

#[test]
fn compiles_unary() {
    let mut compiler = Compiler::new();
    compiler.compile(
        &Expr::Unary {
            operator: UnaryOp::Negate,
            right: Box::new(Expr::Number(5.0)),
        }
    );

    assert_eq!(
        compiler.chunk.code,
        vec![
            OpCode::Constant as u8,
            0,
            OpCode::Negate as u8,
        ]
    );

    assert_eq!(
        compiler.chunk.constants,
        vec![
            Value::Number(5.0)
        ]
    );
}