use std::{
    collections::HashMap,
    process::ExitCode,
    sync::{Arc, RwLock},
};

use crate::token::Value;

#[derive(Debug, Clone)]
pub struct Env {
    values: HashMap<String, Value>,
    enclosing: Option<Arc<RwLock<Env>>>,
}

impl Env {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Env {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), ExitCode> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.write().unwrap().assign(name, value)
        } else {
            eprintln!("Undefined assign variable '{}'.", name);
            return Err(ExitCode::from(70));
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, ExitCode> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.read().unwrap().get(name)
        } else {
            eprintln!("Undefined get variable '{}'.", name);
            return Err(ExitCode::from(70));
        }
    }
}
