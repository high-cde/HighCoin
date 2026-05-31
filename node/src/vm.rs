use crate::zlang_parser::{parse, Ast};
use crate::zlang_tokenizer::tokenize;

#[derive(Debug)]
pub struct Vm {
    pub last_value: i64,
}

impl Vm {
    pub fn new() -> Self {
        Vm { last_value: 0 }
    }

    pub fn deploy(&mut self, _name: &str, _code: &str) -> Result<String, String> {
        Ok("contract deployed".into())
    }

    pub fn call(&mut self, _name: &str, _func: &str, _args: Vec<String>) -> Result<String, String> {
        Ok("call executed".into())
    }

    pub fn execute_zlang(&mut self, code: &str) -> Result<i64, String> {
        let tokens = tokenize(code);
        let ast = parse(&tokens);

        match ast {
            Ast::Number(n) => {
                self.last_value = n;
                Ok(n)
            }
            _ => Ok(0),
        }
    }
}
