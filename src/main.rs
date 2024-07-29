use std::env;
use std::fs;
use std::process::exit;

use interpreter::Scanner;
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }
    let command = &args[1];
    let filename = &args[2];
    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {filename}");
                String::new()
            });

            let scanner = Scanner::new(&file_contents);
            match scanner.tokenize() {
                Ok(tokens) => {
                    for token in tokens {
                        println!("{token}");
                    }
                }
                Err(tokens) => {
                    for token in tokens {
                        println!("{token}");
                    }
                    exit(65)
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
