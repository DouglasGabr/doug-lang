#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
    Equals,
    Let,
    Const,
    OpenParen,
    CloseParen,
    BinaryOperator(char),
    EndOfFile,
    Semicolon,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ';' => tokens.push(Token::Semicolon),
            '+' | '-' | '*' | '/' | '%' => tokens.push(Token::BinaryOperator(c)),
            '=' => tokens.push(Token::Equals),
            ' ' | '\n' | '\t' => continue,
            _ => match c {
                '0'..='9' => {
                    let mut token = String::new();
                    token.push(c);
                    while let Some('0'..='9') = chars.peek() {
                        token.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Number(token));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut token = String::new();
                    token.push(c);
                    while let Some('a'..='z') | Some('A'..='Z') = chars.peek() {
                        token.push(chars.next().unwrap());
                    }
                    match token.as_str() {
                        "let" => tokens.push(Token::Let),
                        "const" => tokens.push(Token::Const),
                        _ => tokens.push(Token::Identifier(token)),
                    }
                }
                _ => todo!("handle character {}", c),
            },
        }
    }

    tokens.push(Token::EndOfFile);
    tokens
}
