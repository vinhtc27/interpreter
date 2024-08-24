use std::{env, fs, process::ExitCode};

mod parser;
use parser::Parser;

mod scanner;
use scanner::Scanner;

mod token;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::SUCCESS;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        String::new()
    });

    let mut scanner = Scanner::new(&file_contents);
    match command.as_str() {
        "tokenize" => {
            if let Err(exitcode) = scanner.tokenize() {
                for token in scanner.tokens() {
                    println!("{}", token);
                }
                exitcode
            } else {
                for token in scanner.tokens() {
                    println!("{}", token);
                }
                ExitCode::SUCCESS
            }
        }
        "parse" => {
            if let Err(exitcode) = scanner.tokenize() {
                return exitcode;
            }
            let mut parser = Parser::new(scanner.tokens(), false);
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let statements = parser.statements();
            for statements in statements {
                println!("{}", statements);
            }
            ExitCode::SUCCESS
        }
        "evaluate" => {
            if let Err(exitcode) = scanner.tokenize() {
                return exitcode;
            }
            let mut parser = Parser::new(scanner.tokens(), false);
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let stmts = parser.statements();
            for stmt in stmts {
                if let Err(exitcode) = stmt.evaluate() {
                    return exitcode;
                }
            }
            ExitCode::SUCCESS
        }
        "run" => {
            if let Err(exitcode) = scanner.tokenize() {
                return exitcode;
            }
            let mut parser = Parser::new(scanner.tokens(), true);
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let stmts = parser.statements();
            for stmt in stmts {
                if let Err(exitcode) = stmt.run() {
                    return exitcode;
                }
            }
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Unknown command: {command}");
            ExitCode::FAILURE
        }
    }
}
