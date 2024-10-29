use std::{
    fmt::Display,
    process::ExitCode,
    sync::{Arc, RwLock},
};

use crate::env::Env;

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

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
    Group(Box<Stmt>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Literal(token) => match &token.token_type {
                TokenType::String(s) => write!(f, "{}", s),
                TokenType::Number(n) => write!(f, "{}", n),
                _ => write!(f, "{}", token.lexeme),
            },
            Expr::Unary(operator, expr) => write!(f, "({} {})", operator.lexeme, expr),
            Expr::Group(stmt) => write!(f, "(group {})", stmt),
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
    pub fn evaluate(&self, environment: Arc<RwLock<Env>>) -> Result<Value, ExitCode> {
        match self {
            Expr::Binary(left, operator, right) => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment.clone())?;
                match (&operator.token_type, &left, &right) {
                    (TokenType::Or, left, right) => match (left, right) {
                        (Value::Boolean(true) | Value::Number(_) | Value::String(_), _) => {
                            Ok(left.clone())
                        }
                        (_, Value::Boolean(true) | Value::Number(_) | Value::String(_)) => {
                            Ok(right.clone())
                        }
                        (_, Value::Nil) => Ok(Value::Boolean(false)),
                        _ => Ok(Value::Boolean(false)),
                    },
                    (TokenType::Plus, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left + right))
                    }
                    (TokenType::Plus, Value::String(left), Value::String(right)) => {
                        Ok(Value::String(left.to_owned() + right))
                    }
                    (TokenType::Plus, _, _) => {
                        eprintln!("Operands must be two numbers or two strings.");
                        Err(ExitCode::from(70))
                    }
                    (TokenType::Minus, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left - right))
                    }
                    (TokenType::Star, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left * right))
                    }
                    (TokenType::Slash, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }
                    (TokenType::Greater, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Boolean(left > right))
                    }
                    (TokenType::GreaterEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Boolean(left >= right))
                    }
                    (TokenType::Less, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Boolean(left < right))
                    }
                    (TokenType::LessEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Boolean(left <= right))
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
                    (TokenType::EqualEqual, left, right) => Ok(Value::Boolean(left == right)),
                    (TokenType::BangEqual, left, right) => Ok(Value::Boolean(left != right)),
                    _ => {
                        eprintln!("Unsupported binary expression.");
                        Err(ExitCode::from(65))
                    }
                }
            }
            Expr::Group(stmt) => stmt.evaluate(environment),
            Expr::Literal(token) => match &token.token_type {
                TokenType::Number(n) => Ok(Value::Number(*n)),
                TokenType::String(s) => Ok(Value::String(s.clone())),
                TokenType::True => Ok(Value::Boolean(true)),
                TokenType::False => Ok(Value::Boolean(false)),
                TokenType::Nil => Ok(Value::Nil),
                TokenType::Identifier => environment.read().unwrap().get(&token.lexeme),
                _ => {
                    eprintln!("Unsupported literal expression.");
                    Err(ExitCode::from(65))
                }
            },
            Expr::Unary(operator, expr) => {
                let expr = expr.evaluate(environment)?;
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
                    _ => {
                        eprintln!("Unsupported unary expression.");
                        Err(ExitCode::from(65))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Print(Box<Stmt>),
    If(Box<Stmt>, Box<Stmt>, Option<Box<Stmt>>),
    Declare(String, Box<Stmt>),
    Assign(String, Box<Stmt>),
    Expr(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                writeln!(f, "{{")?;
                for stmt in stmts {
                    writeln!(f, "   {}", stmt)?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::If(condition, if_branch, else_branch) => {
                write!(f, "if {} {}", condition, if_branch).and_then(|_| {
                    if let Some(else_branch) = else_branch {
                        write!(f, " else {}", else_branch)
                    } else {
                        Ok(())
                    }
                })
            }
            Stmt::Declare(var, expr) => write!(f, "var {} = {}", var, expr),
            Stmt::Assign(var, expr) => write!(f, "{} = {}", var, expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Stmt {
    pub fn evaluate_no_run(&self) -> Result<Value, ExitCode> {
        match self {
            Stmt::Expr(expr) => {
                let value = expr.evaluate(Env::new())?;
                println!("{}", value);
                Ok(value)
            }
            _ => Err(ExitCode::from(65)),
        }
    }

    pub fn evaluate(&self, environment: Arc<RwLock<Env>>) -> Result<Value, ExitCode> {
        match self {
            Stmt::Block(statements) => {
                let block_environment = Env::with_enclosing(environment);
                for stmt in statements {
                    stmt.evaluate(block_environment.clone())?;
                }
                Ok(Value::Nil)
            }
            Stmt::Print(statement) => {
                let value = statement.evaluate(environment)?;
                println!("{}", value);
                Ok(Value::Nil)
            }
            Stmt::If(condition, if_branch, else_branch) => {
                match condition.evaluate(environment.clone())? {
                    Value::Boolean(true) | Value::Number(_) | Value::String(_) => {
                        if_branch.evaluate(environment)
                    }
                    Value::Boolean(false) | Value::Nil => {
                        if let Some(else_branch) = else_branch {
                            else_branch.evaluate(environment)
                        } else {
                            Ok(Value::Nil)
                        }
                    }
                }
            }
            Stmt::Declare(var, expr) => {
                let value = expr.evaluate(environment.clone())?;
                environment.write().unwrap().define(var.clone(), value);
                Ok(Value::Nil)
            }
            Stmt::Assign(var, expr) => {
                let value = expr.evaluate(environment.clone())?;
                environment.write().unwrap().assign(var, value.clone())?;
                Ok(value)
            }
            Stmt::Expr(expr) => expr.evaluate(environment),
        }
    }
}
