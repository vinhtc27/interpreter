use std::process::ExitCode;

use crate::token::{Expr, Token, TokenType};

#[derive(Default)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    exprs: Vec<Expr>,
    current: usize,
    error: bool,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_eof(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_eof() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn express(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.or()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            return Ok(Expr::Unary(equals, Box::new(value)));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ()> {
        let mut expr = self.equality()?;

        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if self.match_tokens(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Ok(Expr::Literal(self.previous()));
        }

        if let TokenType::Number(_) = self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal(self.previous()));
        }

        if let TokenType::String(_) = self.peek().token_type {
            self.advance();
            if let TokenType::String(s) = &self.previous().token_type {
                return Ok(Expr::Literal(Token {
                    token_type: TokenType::String(s.to_string()),
                    lexeme: s.to_string(),
                    line: self.previous().line,
                }));
            }
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.express()?;
            self.consume(TokenType::RightParen, "Unmatched parentheses.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_tokens(&[TokenType::LeftBrace]) {
            let expr = self.express()?;
            self.consume(TokenType::RightBrace, "Unmatched brace.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_tokens(&[
            TokenType::And,
            TokenType::Class,
            TokenType::Else,
            TokenType::For,
            TokenType::Fun,
            TokenType::If,
            TokenType::Or,
            TokenType::Print,
            TokenType::Return,
            TokenType::Super,
            TokenType::This,
            TokenType::Var,
            TokenType::While,
            TokenType::Identifier,
        ]) {
            // self.advance();
            return Ok(Expr::Literal(self.previous()));
        }

        self.advance();
        Err(self.error(self.line(), "Expect expression."))
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_eof() {
            false
        } else {
            &self.peek().token_type == t
        }
    }

    fn line(&self) -> usize {
        self.peek().line
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ()> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            self.error(self.line(), message);
            Err(())
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
        self.error = true;
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            exprs: vec![],
            current: 0,
            error: false,
        }
    }

    pub fn expressions(&self) -> &[Expr] {
        &self.exprs
    }

    pub fn parse(&mut self) -> Result<(), ExitCode> {
        while !self.is_eof() {
            if let Ok(expr) = self.express() {
                self.exprs.push(expr);
            }
        }
        if self.error {
            Err(ExitCode::from(65))
        } else {
            Ok(())
        }
    }
}