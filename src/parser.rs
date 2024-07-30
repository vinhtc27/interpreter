use std::process::ExitCode;

use crate::token::{Expr, Token, TokenType};

#[derive(Default)]
pub struct Parser<'a> {
    tokens: &'a [Token],
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

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
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
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Unmatched parentheses.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_tokens(&[TokenType::LeftBrace]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightBrace, "Unmatched brace.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_tokens(&[TokenType::RightParen]) {
            self.consume(TokenType::RightParen, "Unmatched parentheses.")?;
        }

        if self.match_tokens(&[TokenType::RightBrace]) {
            self.consume(TokenType::RightBrace, "Unmatched parentheses.")?;
        }

        self.advance();
        Ok(Expr::Literal(self.previous()))
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

    fn consume(&mut self, t: TokenType, message: &str) -> Result<Token, ()> {
        if self.check(&t) {
            Ok(self.advance())
        } else {
            self.error(message);
            Err(())
        }
    }

    fn error(&mut self, message: &str) {
        eprintln!("Error: {message}");
        self.error = true;
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            error: false,
        }
    }

    pub fn parse(&mut self) -> ExitCode {
        while !self.is_eof() {
            if let Ok(expr) = self.expression() {
                println!("{}", expr);
            }
        }
        if self.error {
            ExitCode::from(65)
        } else {
            ExitCode::SUCCESS
        }
    }
}
