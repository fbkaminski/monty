use std::fmt::Error;

use lex::token::Token;
use ast::ast::{BinaryExpression, BinaryOp, Number, NumberKind};
use lex::scanner::Scanner;

pub struct Parser<'a> {
    scanner: Scanner<'a>
}

impl Parser<'_> {

    pub fn new<'a>(code: &'a str) -> Parser<'a> {
        Parser {
            scanner: Scanner::new(code)
        }
    }

    pub fn parse(&mut self) -> Result<BinaryExpression, Error> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<BinaryExpression, Error> {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> Result<BinaryExpression, Error> {
        let mut op: BinaryOp = BinaryOp::Subtraction;
        let mut left = Number {kind: NumberKind::Integer, value: String::new()};
        let mut right = Number {kind: NumberKind::Integer, value: String::new()};
        let mut number_offset = 0;
        loop {
            let tok = self.scanner.next_token();
            match tok {
                Token::Add => { op = BinaryOp::Addition },
                Token::Mul => { op = BinaryOp::Multiplication },
                Token::Div => { op = BinaryOp::Division },
                Token::Sub => { op = BinaryOp::Subtraction },
                Token::Number {ref value} => {
                    if number_offset == 0 {
                        left = Number{kind: NumberKind::Integer, value: value.clone()};
                    } else {
                        right = Number{kind: NumberKind::Integer, value: value.clone()};
                    }
                    number_offset += 1;
                },
                Token::Eos => { break },
                _ => {println!("unhandled case"); break;}
            }
        }
        Ok(BinaryExpression{
            op: op,
            left: left,
            right: right
        })
    }

    // fn parse_unary_expression(&mut self) {

    // }

    // fn current(&self) -> Option<Token> {
    //     None
    // }

    // fn advance(&mut self) {

    // }

}