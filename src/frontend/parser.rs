use std::iter::Peekable;
use std::vec::IntoIter;

use super::ast::{Expression, Program, Statement};
use super::lexer::{tokenize, Token};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(source_code: &str) -> Self {
        Self {
            tokens: tokenize(source_code).into_iter().peekable(),
        }
    }
    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program { body: Vec::new() };
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::EndOfFile => break,
                _ => program.body.push(self.parse_statement()),
            }
        }
        program
    }

    fn parse_statement(&mut self) -> Statement {
        Statement::Expression(self.parse_expression())
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expression();
        while let Some(Token::BinaryOperator('+') | Token::BinaryOperator('-')) = self.tokens.peek()
        {
            let operator = self.tokens.next().unwrap();
            let right = self.parse_multiplicative_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator: match operator {
                    Token::BinaryOperator(operator) => operator,
                    _ => unreachable!(),
                },
            };
        }
        left
    }

    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_primary_expression();
        while let Some(
            Token::BinaryOperator('*') | Token::BinaryOperator('/') | Token::BinaryOperator('%'),
        ) = self.tokens.peek()
        {
            let operator = self.tokens.next().unwrap();
            let right = self.parse_primary_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator: match operator {
                    Token::BinaryOperator(operator) => operator,
                    _ => unreachable!(),
                },
            };
        }
        left
    }

    fn parse_primary_expression(&mut self) -> Expression {
        match self.tokens.peek().unwrap() {
            Token::Identifier(_) => match self.tokens.next().unwrap() {
                Token::Identifier(identifier) => Expression::Identifier(identifier),
                _ => unreachable!(),
            },
            Token::Number(_) => match self.tokens.next().unwrap() {
                Token::Number(number) => Expression::NumericLiteral(number.parse().unwrap()),
                _ => unreachable!(),
            },
            Token::OpenParen => {
                self.tokens.next();
                let expression = self.parse_expression();
                match self.tokens.next() {
                    Some(Token::CloseParen) => expression,
                    Some(token) => panic!("expected ')' but got {:?}", token),
                    _ => unreachable!(),
                }
            }
            token => panic!("unexpected token: {:?}", token),
        }
    }
}
