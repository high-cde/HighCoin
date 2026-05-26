use serde_json::Value;
use std::collections::HashMap;

use crate::zlang_tokenizer::tokenize;
use crate::zlang_parser::parse;
use crate::zlang_vm::eval;

#[derive(Clone)]
pub struct Contract {
    pub name: String,
    pub code: String,
}

pub struct Vm {
    contracts: HashMap<String, Contract>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            contracts: HashMap::new(),
        }
    }

    /// Deploy a contract with a logical name and code identifier
    pub fn deploy(&mut self, name: &str, code: &str) -> Result<Value, String> {
        if self.contracts.contains_key(name) {
            return Err(format!("contract '{}' already exists", name));
        }

        let c = Contract {
            name: name.to_string(),
            code: code.to_string(),
        };

        self.contracts.insert(name.to_string(), c);
        Ok(Value::String(format!("deployed: {}", name)))
    }

    /// Call a deployed contract's function with JSON args
    pub fn call(&mut self, name: &str, func: &str, input: Value) -> Result<Value, String> {
        let c = self
            .contracts
            .get(name)
            .ok_or_else(|| format!("contract '{}' not found", name))?;

        match (c.code.as_str(), func) {
            ("math", "add") => {
                let a = input["a"].as_i64().unwrap_or(0);
                let b = input["b"].as_i64().unwrap_or(0);
                Ok(Value::from(a + b))
            }
            ("math", "mul") => {
                let a = input["a"].as_i64().unwrap_or(1);
                let b = input["b"].as_i64().unwrap_or(1);
                Ok(Value::from(a * b))
            }
            ("log", "emit") => {
                let msg = input["msg"].as_str().unwrap_or("");
                Ok(Value::String(format!("LOG: {}", msg)))
            }
            _ => Err(format!("unknown func '{}' for contract '{}'", func, name)),
        }
    }

    /// Backwards-compatible execute method for simple operations
    pub fn execute(&mut self, code: &str, input: Value) -> Result<Value, String> {
        if code == "add" {
            let a = input["a"].as_i64().unwrap_or(0);
            let b = input["b"].as_i64().unwrap_or(0);
            return Ok(Value::from(a + b));
        }
        Ok(Value::String("ok".into()))
    }

    /// Execute a Z‑Lang expression: tokenize -> parse -> eval
    pub fn execute_zlang(&mut self, code: &str) -> Result<Value, String> {
        // Tokenize
        let tokens = tokenize(code);
        // Parse tokens into AST
        let ast = parse(&tokens).map_err(|e| format!("parse error: {}", e))?;
        // Evaluate AST
        let result = eval(&ast);
        Ok(Value::from(result))
    }
}
