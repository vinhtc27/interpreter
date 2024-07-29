use std::fmt::Display;
use std::str::Chars;

#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
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
    // One or two character tokens
    //TODO:
    // Literals
    //TODO
    // Keywords
    //TODO
    // End Of File
    EOF,
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
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {}

impl Display for Literal {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    #[allow(dead_code)]
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref literal) = self.literal {
            write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
        } else {
            write!(f, "{} {} null", self.token_type, self.lexeme)
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    chars: Chars<'a>,
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
            chars: source.chars(),
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

    fn peak(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Returns source text at `start..current`
    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.lexeme().to_string(),
            literal,
            line: self.line,
        })
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, Vec<Token>> {
        while let Some(c) = self.advance() {
            self.start = self.current - c.len_utf8();
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::SemiColon, None),
                '*' => self.add_token(TokenType::Star, None),
                '=' => {
                    if self.peak() == Some('=') {
                        self.advance();
                        self.add_token(TokenType::EqualEqual, None);
                    } else {
                        self.add_token(TokenType::Equal, None);
                    }
                }
                '!' => {
                    if self.peak() == Some('=') {
                        self.advance();
                        self.add_token(TokenType::BangEqual, None);
                    } else {
                        self.add_token(TokenType::Bang, None);
                    }
                }
                '<' => {
                    if self.peak() == Some('=') {
                        self.advance();
                        self.add_token(TokenType::LessEqual, None);
                    } else {
                        self.add_token(TokenType::Less, None);
                    }
                }
                '>' => {
                    if self.peak() == Some('=') {
                        self.advance();
                        self.add_token(TokenType::GreaterEqual, None);
                    } else {
                        self.add_token(TokenType::Greater, None);
                    }
                }
                '/' => {
                    //? Comment
                    if self.peak() == Some('/') {
                        while self.peak() != Some('\n') && self.peak().is_some() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
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
            token_type: TokenType::EOF,
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
