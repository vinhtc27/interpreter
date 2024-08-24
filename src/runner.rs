use std::{collections::HashMap, process::ExitCode};

use crate::token::{Expr, Stmt};

pub struct Runner<'a> {
    stmts: &'a [Stmt],
}

impl<'a> Runner<'a> {
    pub fn new(stmts: &'a [Stmt]) -> Self {
        Self { stmts }
    }

    pub fn run(&self) -> Result<(), ExitCode> {
        let mut vars: HashMap<String, Expr> = HashMap::new();
        for stmt in self.stmts {
            match stmt {
                Stmt::Expr(expr) => {
                    expr.evaluate()?;
                }
                Stmt::Print(expr) => {
                    let value = if let Some(var) = vars.get(&expr.to_string()) {
                        var.evaluate()?
                    } else {
                        expr.evaluate()?
                    };
                    println!("{}", value);
                }
                Stmt::Var(var, expr) => {
                    vars.insert(var.clone(), expr.clone());
                }
            }
        }
        Ok(())
    }
}
