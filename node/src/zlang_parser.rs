use crate::zlang_tokenizer::{Token};

#[derive(Debug)]
pub enum ZAst {
    Number(i64),
    Add(Box<ZAst>, Box<ZAst>),
}

pub fn parse(tokens: &[Token]) -> Result<ZAst, String> {
    if tokens.len() == 3 {
        if let Token::Number(a) = tokens[0].clone() {
            if let Token::Plus = tokens[1] {
                if let Token::Number(b) = tokens[2].clone() {
                    return Ok(ZAst::Add(Box::new(ZAst::Number(a)), Box::new(ZAst::Number(b))));
                }
            }
        }
    }

    Err("parse error".into())
}
