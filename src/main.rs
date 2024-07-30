use std::env;
use std::fs;
use std::process::ExitCode;

use interpreter::Interpreter;
mod interpreter;

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

    let mut interpreter = Interpreter::new(&file_contents);
    match command.as_str() {
        "tokenize" => interpreter.tokenize(true),
        "parse" => interpreter.parse(),
        _ => {
            eprintln!("Unknown command: {command}");
            ExitCode::FAILURE
        }
    }
}
