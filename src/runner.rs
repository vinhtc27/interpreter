use std::{collections::HashMap, process::ExitCode};

use crate::token::{Stmt, Value};

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
                    Stmt::Print(_) | Stmt::Declare(_, _) | Stmt::Assign(_, _) => unreachable!(),
                    Stmt::Expr(expr) => expr.evaluate(&HashMap::new())?,
                }
            );
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), ExitCode> {
        let mut vars: HashMap<String, Value> = HashMap::new();
        for stmt in self.stmts {
            stmt.evaluate(&mut vars)?;
        }
        Ok(())
    }
}
