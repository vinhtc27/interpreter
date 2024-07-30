use std::fmt::Display;

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
    pub fn evaluate(&self) -> Value {
        match self {
            Expr::Binary(left, operator, right) => {
                let left = left.evaluate();
                let right = right.evaluate();
                match (&operator.token_type, left, right) {
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (TokenType::Plus, Value::String(l), Value::String(r)) => Value::String(l + &r),
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                    (TokenType::Star, Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                    (TokenType::Slash, Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                    (TokenType::Greater, Value::Number(l), Value::Number(r)) => {
                        Value::Boolean(l > r)
                    }
                    (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Boolean(l >= r)
                    }
                    (TokenType::Less, Value::Number(l), Value::Number(r)) => Value::Boolean(l < r),
                    (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Boolean(l <= r)
                    }
                    (TokenType::EqualEqual, l, r) => Value::Boolean(l == r),
                    (TokenType::BangEqual, l, r) => Value::Boolean(l != r),
                    _ => panic!("Invalid operator: {:?}", operator),
                }
            }
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(token) => match &token.token_type {
                TokenType::Number(n) => Value::Number(*n),
                TokenType::String(s) => Value::String(s.clone()),
                TokenType::True => Value::Boolean(true),
                TokenType::False => Value::Boolean(false),
                TokenType::Nil => Value::Nil,
                _ => panic!("Invalid literal: {:?}", token),
            },
            Expr::Unary(operator, expr) => {
                let expr = expr.evaluate();
                match operator.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = expr {
                            Value::Number(-n)
                        } else {
                            panic!("Invalid operand for unary minus: {:?}", expr)
                        }
                    }
                    TokenType::Bang => {
                        if let Value::Boolean(b) = expr {
                            Value::Boolean(!b)
                        } else if let Value::Number(_) = expr {
                            Value::Boolean(false)
                        } else if let Value::Nil = expr {
                            Value::Boolean(true)
                        } else {
                            panic!("Invalid operand for unary bang: {:?}", expr)
                        }
                    }
                    _ => panic!("Invalid operator: {:?}", operator),
                }
            }
        }
    }
}
