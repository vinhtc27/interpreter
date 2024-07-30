use std::iter::Peekable;
use std::process::ExitCode;
use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error: bool,
}

impl<'a> Scanner<'a> {
    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.current += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.lexeme().to_string(),
            line: self.line,
        })
    }

    fn error(&mut self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
        self.error = true;
    }

    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            error: false,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn tokenize(&mut self) -> Result<(), ExitCode> {
        while let Some(c) = self.advance() {
            self.start = self.current - c.len_utf8();
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::SemiColon),
                '*' => self.add_token(TokenType::Star),
                '=' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '!' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.add_token(TokenType::BangEqual);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                }
                '<' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                '/' => {
                    //? Comment
                    if self.peek() == Some(&'/') {
                        while self.peek() != Some(&'\n') && self.peek().is_some() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                '"' => {
                    while self.peek() != Some(&'"') && self.peek().is_some() {
                        if self.peek() == Some(&'\n') {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    if self.peek().is_none() {
                        self.error(self.line, "Unterminated string.");
                    } else {
                        self.advance();
                        self.add_token(TokenType::String(
                            self.lexeme()[1..self.lexeme().len() - 1].to_string(),
                        ));
                    }
                }
                c if c.is_ascii_digit() => {
                    while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                        self.advance();
                    }

                    if self.peek() == Some(&'.')
                        && self
                            .chars
                            .clone()
                            .nth(1)
                            .map_or(false, |c| c.is_ascii_digit())
                    {
                        self.advance();
                        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                            self.advance();
                        }
                    }

                    self.add_token(TokenType::Number(self.lexeme().parse().unwrap()));
                }
                c if c.is_alphabetic() || c == '_' => {
                    while self
                        .peek()
                        .map_or(false, |c| c.is_alphanumeric() || c == &'_')
                    {
                        self.advance();
                    }

                    let lexeme = self.lexeme();
                    let token_type = match lexeme {
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "for" => TokenType::For,
                        "fun" => TokenType::Fun,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While,
                        _ => TokenType::Identifier,
                    };

                    self.add_token(token_type);
                }
                '\n' => self.line += 1,
                c if c.is_whitespace() => {}
                _ => self.error(self.line, &format!("Unexpected character: {c}")),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
        });

        if self.error {
            Err(ExitCode::from(65))
        } else {
            Ok(())
        }
    }
}
