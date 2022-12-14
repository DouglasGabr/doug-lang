#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Program(Program),
    VariableDeclaration {
        constant: bool,
        identifier: String,
        value: Option<Expression>,
    },
}

#[derive(Debug)]
pub enum Expression {
    BinaryExpression {
        left: Box<Expression>,
        operator: char,
        right: Box<Expression>,
    },
    Identifier(String),
    NumericLiteral(f64),
}
