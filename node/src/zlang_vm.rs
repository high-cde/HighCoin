use crate::zlang_parser::ZAst;

pub fn eval(ast: &ZAst) -> i64 {
    match ast {
        ZAst::Number(n) => *n,
        ZAst::Add(a, b) => eval(a) + eval(b),
    }
}

