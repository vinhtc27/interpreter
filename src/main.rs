use std::env;
use std::fs;
use std::process::ExitCode;

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
        "run" => {
            if let Err(exitcode) = scanner.tokenize() {
                return exitcode;
            }
            let mut parser = Parser::new(scanner.tokens());
            if let Err(exitcode) = parser.parse() {
                return exitcode;
            }
            let statements = parser.statements();
            for statement in statements {
                match statement.evaluate() {
                    Ok(value) => println!("{}", value),
                    Err(exitcode) => return exitcode,
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
