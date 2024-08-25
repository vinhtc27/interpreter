use std::{collections::HashMap, process::ExitCode};

use crate::token::Value;

#[derive(Debug, Clone)]
pub struct Env {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Env) -> Self {
        Env {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), ExitCode> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(name, value)
        } else {
            eprintln!("Undefined variable '{}'.", name);
            return Err(ExitCode::from(70));
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, ExitCode> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.get(name)
        } else {
            eprintln!("Undefined variable '{}'.", name);
            return Err(ExitCode::from(70));
        }
    }
}
