#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Plus,
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for part in src.split_whitespace() {
        if part == "+" {
            tokens.push(Token::Plus);
        } else if let Ok(n) = part.parse::<i64>() {
            tokens.push(Token::Number(n));
        }
    }

    tokens
}
