#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for part in input.split_whitespace() {
        match part {
            "+" => tokens.push(Token::Plus),
            "-" => tokens.push(Token::Minus),
            "*" => tokens.push(Token::Star),
            "/" => tokens.push(Token::Slash),
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            n if n.parse::<i64>().is_ok() => tokens.push(Token::Number(n.parse().unwrap())),
            id => tokens.push(Token::Ident(id.to_string())),
        }
    }

    tokens
}
