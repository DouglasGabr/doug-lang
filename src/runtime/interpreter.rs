use crate::frontend::ast::{Expression, Statement};

use super::values::RuntimeValue;

pub fn evaluate(ast_node: Statement) -> RuntimeValue {
    match ast_node {
        Statement::Program(program) => {
            let mut last_evaluated = RuntimeValue::Null;
            for statement in program.body {
                last_evaluated = evaluate(statement);
            }
            last_evaluated
        }
        Statement::Expression(expression) => match expression {
            Expression::NumericLiteral(val) => RuntimeValue::Number(val),
            Expression::NullLiteral => RuntimeValue::Null,
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_hand_side = evaluate(Statement::Expression(*left));
                let right_hand_side = evaluate(Statement::Expression(*right));
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
            expression => unimplemented!("{:?}", expression),
        },
    }
}
