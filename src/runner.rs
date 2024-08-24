use std::{collections::HashMap, process::ExitCode};

use crate::token::{Expr, Stmt};

pub struct Runner<'a> {
    stmts: &'a [Stmt],
}

impl<'a> Runner<'a> {
    pub fn new(stmts: &'a [Stmt]) -> Self {
        Self { stmts }
    }

    pub fn evaluate(&self) -> Result<(), ExitCode> {
        for stmt in self.stmts {
            println!(
                "{}",
                match stmt {
                    Stmt::Expr(expr) => expr.evaluate(&HashMap::new())?,
                    Stmt::Print(_) | Stmt::Var(_, _) => unreachable!(),
                }
            );
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), ExitCode> {
        let mut vars: HashMap<String, Expr> = HashMap::new();
        for stmt in self.stmts {
            match stmt {
                Stmt::Expr(expr) => {
                    expr.evaluate(&vars)?;
                }
                Stmt::Print(expr) => {
                    let value = if let Some(var) = vars.get(&expr.to_string()) {
                        var.evaluate(&vars)?
                    } else {
                        expr.evaluate(&vars)?
                    };
                    println!("{}", value);
                }
                Stmt::Var(var, expr) => {
                    if *var != expr.to_string() {
                        vars.insert(var.clone(), expr.clone());
                    }
                }
            }
        }
        Ok(())
    }
}
