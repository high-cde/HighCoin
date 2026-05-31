use crate::zlang_tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Ast {
    Number(i64),
    Ident(String),
    BinaryOp {
        left: Box<Ast>,
        op: String,
        right: Box<Ast>,
    },
}

pub fn parse(tokens: &[Token]) -> Ast {
    // Parser minimale: restituisce solo il primo token
    match tokens.first() {
        Some(Token::Number(n)) => Ast::Number(*n),
        Some(Token::Ident(s)) => Ast::Ident(s.clone()),
        _ => Ast::Number(0),
    }
}
