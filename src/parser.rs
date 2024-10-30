use std::process::ExitCode;

use crate::token::{Expr, Stmt, Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token],
    stmts: Vec<Stmt>,
    current: usize,
    reporter: ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            stmts: vec![],
            current: 0,
            reporter: ErrorReporter::new(),
        }
    }

    pub fn statements(&mut self) -> &mut [Stmt] {
        &mut self.stmts
    }

    pub fn parse(&mut self) -> Result<(), ExitCode> {
        while !self.is_eof() {
            if let Ok(stmt) = self.parse_statement() {
                self.stmts.push(stmt);
            }
        }
        if self.reporter.had_error {
            Err(ExitCode::from(65))
        } else {
            Ok(())
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, ()> {
        if self.match_tokens(&[TokenType::LeftBrace]) {
            self.block_statement()
        } else if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_tokens(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_tokens(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_tokens(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_tokens(&[TokenType::Var]) {
            self.declare_statement()
        } else if self.match_tokens(&[TokenType::Identifier]) {
            self.assign_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, ()> {
        let mut stmts = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_eof() {
            stmts.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' .")?;

        if stmts.is_empty() {
            return Err(());
        }

        Ok(Stmt::Block(stmts))
    }

    fn print_statement(&mut self) -> Result<Stmt, ()> {
        let stmt = self.parse_statement()?;
        if self.peek().token_type == TokenType::SemiColon {
            self.consume(TokenType::SemiColon, "")?;
        }
        Ok(Stmt::Print(Box::new(stmt)))
    }

    fn while_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_statement()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = self.parse_statement()?;
        Ok(Stmt::While(Box::new(condition), Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.match_tokens(&[TokenType::SemiColon]) {
            None
        } else if self.match_tokens(&[TokenType::Var]) {
            Some(self.declare_statement()?)
        } else if self.match_tokens(&[TokenType::Identifier]) {
            Some(self.assign_statement()?)
        } else {
            Some(self.parse_statement()?)
        };
        let condition = if self.match_tokens(&[TokenType::SemiColon]) {
            None
        } else {
            Some(self.parse_statement()?)
        };
        let increment = if self.match_tokens(&[TokenType::RightParen]) {
            None
        } else {
            Some(self.parse_statement()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let body = self.parse_statement()?;
        Ok(Stmt::For(
            initializer.map(Box::new),
            condition.map(Box::new),
            increment.map(Box::new),
            Box::new(body),
        ))
    }

    fn if_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.parse_statement()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
        let then_branch = self.parse_statement()?;
        let else_branch = if self.match_tokens(&[TokenType::Else]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Stmt::If(
            Box::new(condition),
            Box::new(then_branch),
            else_branch,
        ))
    }

    fn declare_statement(&mut self) -> Result<Stmt, ()> {
        if !self.check(&TokenType::Identifier) {
            let token = self.previous();
            self.reporter
                .error(token.line, &token.lexeme, "Expect expression.");
            return Err(());
        }

        let var = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let stmt = if self.match_tokens(&[TokenType::Equal]) {
            self.parse_statement()?
        } else {
            let token = self.peek();
            if token.token_type == TokenType::SemiColon {
                let var_token = &self.tokens[self.current - 2];
                self.reporter
                    .error(var_token.line, "var", "Expect expression.");
                return Err(());
            }
            self.expression_statement()?
        };

        if self.peek().token_type == TokenType::SemiColon {
            self.consume(TokenType::SemiColon, "")?;
        }
        Ok(Stmt::Declare(var.lexeme, Box::new(stmt)))
    }

    fn assign_statement(&mut self) -> Result<Stmt, ()> {
        let var = self.previous();
        match self.peek().token_type {
            TokenType::SemiColon => {
                self.consume(TokenType::SemiColon, "")?;
                Ok(Stmt::Expr(Expr::Literal(var)))
            }
            TokenType::Equal => {
                self.consume(TokenType::Equal, "")?;
                let stmt = self.parse_statement()?;
                Ok(Stmt::Assign(var.lexeme, Box::new(stmt)))
            }
            _ => {
                self.retreat();
                Ok(self.expression_statement()?)
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ()> {
        let expr = self.express()?;
        if self.peek().token_type == TokenType::SemiColon {
            self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        }
        Ok(Stmt::Expr(expr))
    }

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

    fn retreat(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    fn express(&mut self) -> Result<Expr, ()> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.or()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
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
            let stmt = self.parse_statement()?;
            self.consume(TokenType::RightParen, "Unmatched parentheses.")?;
            return Ok(Expr::Group(Box::new(stmt)));
        }

        if self.match_tokens(&[TokenType::LeftBrace]) {
            let stmt = self.parse_statement()?;
            self.consume(TokenType::RightBrace, "Unmatched brace.")?;
            return Ok(Expr::Group(Box::new(stmt)));
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
            return Ok(Expr::Literal(self.previous()));
        }

        let token = self.advance();
        self.reporter
            .error(token.line, &token.lexeme, "Expect expression.");
        Err(())
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ()> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let token = self.advance();
            self.reporter.error(token.line, &token.lexeme, message);
            Err(())
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, token: &str, message: &str) {
        self.report(line, token, message);
        self.had_error = true;
    }

    fn report(&self, line: usize, token: &str, message: &str) {
        eprintln!("[line {}] Error at '{}': {}", line, token, message);
    }
}
