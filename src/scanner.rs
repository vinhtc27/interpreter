use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Star,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    String,
    Number,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::SemiColon => write!(f, "SEMICOLON"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref literal) = self.literal {
            match literal {
                Literal::String(s) => write!(f, "{} {} {}", self.token_type, self.lexeme, s),
                Literal::Number(n) => write!(f, "{} {} {:?}", self.token_type, self.lexeme, n),
            }
        } else {
            write!(f, "{} {} null", self.token_type, self.lexeme)
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    report: ScanReport,
}

impl<'a> Scanner<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            report: ScanReport::default(),
        }
    }

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

    /// Returns source text at `start..current`
    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.lexeme();

        let literal = if token_type == TokenType::String {
            Some(Literal::String(lexeme[1..lexeme.len() - 1].to_string()))
        } else if token_type == TokenType::Number {
            Some(Literal::Number(lexeme.parse().unwrap()))
        } else {
            None
        };

        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line: self.line,
        })
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, Vec<Token>> {
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
                        self.report.error(self.line, "Unterminated string.");
                        break;
                    }

                    self.advance();
                    self.add_token(TokenType::String);
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

                    self.add_token(TokenType::Number);
                }
                '\n' => self.line += 1,
                c if c.is_whitespace() => {}
                _ => self
                    .report
                    .error(self.line, &format!("Unexpected character: {c}")),
            }
        }

        // NOTE: specifically not using `add_non_lit` to trim trailing newline for the lexeme
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        if self.report.had_error {
            Err(self.tokens)
        } else {
            Ok(self.tokens)
        }
    }
}

#[derive(Default)]
pub struct ScanReport {
    had_error: bool,
}

trait Report {
    fn report(&mut self, line: usize, location: &str, msg: &str);
    fn error(&mut self, line: usize, msg: &str);
}

impl Report for ScanReport {
    fn report(&mut self, line: usize, location: &str, msg: &str) {
        eprintln!("[line {line}] Error{location}: {msg}");
        self.had_error = true;
    }

    #[inline]
    fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg)
    }
}
