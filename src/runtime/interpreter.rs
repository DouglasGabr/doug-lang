use crate::frontend::ast::{Expression, Statement};

use super::{environment::Environment, values::RuntimeValue};

pub fn evaluate(ast_node: Statement, environment: &mut Environment) -> RuntimeValue {
    match ast_node {
        Statement::Program(program) => {
            let mut last_evaluated = RuntimeValue::Null;
            for statement in program.body {
                last_evaluated = evaluate(statement, environment);
            }
            last_evaluated
        }
        Statement::Expression(expression) => match expression {
            Expression::NumericLiteral(val) => RuntimeValue::Number(val),
            Expression::Identifier(symbol) => environment.lookup_variable(&symbol),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_hand_side = evaluate(Statement::Expression(*left), environment);
                let right_hand_side = evaluate(Statement::Expression(*right), environment);
                match (left_hand_side, right_hand_side) {
                    (RuntimeValue::Number(left), RuntimeValue::Number(right)) => match operator {
                        '+' => RuntimeValue::Number(left + right),
                        '-' => RuntimeValue::Number(left - right),
                        '*' => RuntimeValue::Number(left * right),
                        '/' => RuntimeValue::Number(left / right),
                        '%' => RuntimeValue::Number(left % right),
                        _ => unreachable!(),
                    },
                    _ => todo!("evaluate non number binary expression"),
                }
            }
        },
    }
}
