use crate::lexer;
use crate::token;
use crate::ast;
use std::collections::HashMap;

type PrefixParse = dyn Fn() -> Box<dyn ast::Node>;
type InfixParse = dyn Fn(Box<dyn ast::Node>) -> Box<dyn ast::Node>;

pub struct Parser {
    lexer: lexer::Lexer,
    errors: Vec<String>,
    current_token: token::Token,
    peek_token: token::Token,
    prefix_parsers: HashMap<token::TokenType, Box<PrefixParse>>,
    infix_parsers: HashMap<token::TokenType, Box<InfixParse>>,
}

impl Parser {
    pub fn new(mut lx: lexer::Lexer) -> Self {
        let next = lx.next_token();
        let peek = lx.next_token();
        Self {
            lexer: lx,
            errors: vec![],
            current_token: next,
            peek_token: peek,
            prefix_parsers: HashMap::new(),
            infix_parsers: HashMap::new()
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, token: token::TokenType) -> bool {
        if self.peek_token.token_type == token {
            self.next_token();
            return true
        }

        false
    }

    fn peek_token_is(&self, token: token::TokenType) -> bool {
        self.peek_token.token_type == token
    }

    fn current_token_is(&self, token: token::TokenType) -> bool {
        self.current_token.token_type == token
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, &'static str> {
        let mut program = ast::Program::new();

        while self.current_token.token_type != token::TokenType::Eof {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => return Err(e)
            }

            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn ast::Node>, &'static str> {
        match self.current_token.token_type {
            token::TokenType::Let => self.parse_let(),
            token::TokenType::Return => self.parse_return(),
            token::TokenType::If => self.parse_if(),
            _ => Err("Invalid statement")
        }
    }

    fn parse_let(&mut self) -> Result<Box<dyn ast::Node>, &'static str> {
        let mut stmt = Box::new(ast::LetStatement::new(self.current_token.clone()));

        if !self.expect_peek(token::TokenType::Identifier) {
            return Err("Invalid let statement: expecting identifier");
        }

        stmt.name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone()
        };

        if !self.expect_peek(token::TokenType::Assign) {
            return Err("Invalid let statement: expecting assign (=)");
        }

        while !self.current_token_is(token::TokenType::Semicolon) {
            self.next_token();
        }

        Ok(stmt)
    }

    fn parse_return(&mut self) -> Result<Box<dyn ast::Node>, &'static str> {
        let stmt = Box::new(ast::ReturnStatement::new(self.current_token.clone()));

        self.next_token();

        while !self.current_token_is(token::TokenType::Semicolon) {
            self.next_token();
        }

        Ok(stmt)
    }

    fn parse_if(&mut self) -> Result<Box<dyn ast::Node>, &'static str> {
        Err("Invalid if statement")
    }
}
