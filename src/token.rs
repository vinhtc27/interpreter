use std::{fmt::Display, process::ExitCode};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //? Characters: (, ), {, }, ,, ., -, +, ;, *, =, ==, !, !=, <, <=, >, >=, /
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
    //? Literals:
    String(String),
    Number(f64),
    //? Identifier
    Identifier,
    //? Reserved Words: and, class, else, false, for, fun, if, nil, or, print, return, super, this, true, var, while
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //? End of file
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
            TokenType::String(_) => write!(f, "STRING"),
            TokenType::Number(_) => write!(f, "NUMBER"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::For => write!(f, "FOR"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::String(s) => write!(f, "{} {} {}", self.token_type, self.lexeme, s),
            TokenType::Number(n) => write!(f, "{} {} {:?}", self.token_type, self.lexeme, n),
            _ => write!(f, "{} {} null", self.token_type, self.lexeme),
        }
    }
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
            Expr::Literal(token) => match &token.token_type {
                TokenType::String(s) => write!(f, "{}", s),
                TokenType::Number(n) => write!(f, "{:?}", n),
                _ => write!(f, "{}", token.lexeme),
            },
            Expr::Unary(operator, expr) => write!(f, "({} {expr})", operator.lexeme),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Expr {
    fn evaluate(&self) -> Result<Value, ExitCode> {
        match self {
            Expr::Binary(left, operator, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                match (&operator.token_type, left, right) {
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l + r))
                    }
                    (TokenType::Plus, Value::String(l), Value::String(r)) => {
                        Ok(Value::String(l + &r))
                    }
                    (TokenType::Plus, _, _) => {
                        eprintln!("Operands must be two numbers or two strings.");
                        Err(ExitCode::from(70))
                    }
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (TokenType::Star, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }
                    (TokenType::Slash, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l / r))
                    }
                    (TokenType::Greater, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l > r))
                    }
                    (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l >= r))
                    }
                    (TokenType::Less, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l < r))
                    }
                    (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l <= r))
                    }
                    (
                        TokenType::Minus
                        | TokenType::Star
                        | TokenType::Slash
                        | TokenType::Greater
                        | TokenType::GreaterEqual
                        | TokenType::Less
                        | TokenType::LessEqual,
                        _,
                        _,
                    ) => {
                        eprintln!("Operand must be a number.");
                        Err(ExitCode::from(70))
                    }
                    (TokenType::EqualEqual, l, r) => Ok(Value::Boolean(l == r)),
                    (TokenType::BangEqual, l, r) => Ok(Value::Boolean(l != r)),
                    _ => Err(ExitCode::from(65)),
                }
            }
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(token) => match &token.token_type {
                TokenType::Number(n) => Ok(Value::Number(*n)),
                TokenType::String(s) => Ok(Value::String(s.clone())),
                TokenType::True => Ok(Value::Boolean(true)),
                TokenType::False => Ok(Value::Boolean(false)),
                TokenType::Nil => Ok(Value::Nil),
                _ => Err(ExitCode::from(65)),
            },
            Expr::Unary(operator, expr) => {
                let expr = expr.evaluate()?;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = expr {
                            Ok(Value::Number(-n))
                        } else {
                            eprintln!("Operand must be a number.");
                            Err(ExitCode::from(70))
                        }
                    }
                    TokenType::Bang => {
                        if let Value::Boolean(b) = expr {
                            Ok(Value::Boolean(!b))
                        } else if let Value::Number(_) = expr {
                            Ok(Value::Boolean(false))
                        } else if let Value::Nil = expr {
                            Ok(Value::Boolean(true))
                        } else {
                            eprintln!("Operand must be a number or boolean.");
                            Err(ExitCode::from(65))
                        }
                    }
                    _ => Err(ExitCode::from(65)),
                }
            }
        }
    }
}

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {};", expr),
        }
    }
}

impl Stmt {
    pub fn evaluate(&self) -> Result<(), ExitCode> {
        println!(
            "{}",
            match self {
                Stmt::Expr(expr) => expr.evaluate()?,
                Stmt::Print(expr) => expr.evaluate()?,
            }
        );

        Ok(())
    }

    pub fn run(&self) -> Result<(), ExitCode> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate()?;
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate()?;
                println!("{}", value);
            }
        }
        Ok(())
    }
}
