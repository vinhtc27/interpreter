use std::{env as StdEnv, fs, process::ExitCode};

mod parser;
use parser::Parser;

mod env;
use env::Env;

mod scanner;
use scanner::Scanner;

mod token;

fn main() -> ExitCode {
    let args = StdEnv::args().collect::<Vec<_>>();
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
            let mut parser = Parser::new(scanner.tokens());
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
            let mut parser = Parser::new(scanner.tokens());
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let statements = parser.statements();
            for statement in statements {
                if let Err(exitcode) = statement.evaluate_no_run() {
                    return exitcode;
                }
            }
            ExitCode::SUCCESS
        }
        "run" => {
            if let Err(exitcode) = scanner.tokenize() {
                return exitcode;
            }
            let mut parser = Parser::new(scanner.tokens());
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let environment = Env::new();
            let statements = parser.statements();
            for statement in statements {
                if let Err(exitcode) = statement.evaluate(environment.clone()) {
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
