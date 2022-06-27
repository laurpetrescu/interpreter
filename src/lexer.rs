use crate::token;

pub fn test_next_token(input_string: String) {
    let mut lx = Lexer::new(input_string);
    let mut tok = lx.next_token();
    while tok.token_type != token::TokenType::Eof {
        println!("{:?}", tok);
        tok = lx.next_token();
    }
}

lazy_static::lazy_static! {
static ref KEYWORDS: std::collections::HashMap<&'static str, token::TokenType> = {
    let mut map = std::collections::HashMap::new();
    map.insert(token::FUNCTION, token::TokenType::Function);
    map.insert(token::LET,      token::TokenType::Let);
    map.insert(token::TRUE,     token::TokenType::True);
    map.insert(token::FALSE,    token::TokenType::False);
    map.insert(token::IF,       token::TokenType::If);
    map.insert(token::ELSE,     token::TokenType::Else);
    map.insert(token::RETURN,   token::TokenType::Return);
    map.insert(token::KAKA,     token::TokenType::Kaka);
    map.insert(token::MACA,     token::TokenType::Maca);
    
    map
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1, 		// ==
    LessGreater = 2, 	// > or <
    Sum = 3,			// +
    Product = 4,		// *
    Prefix = 5,			// -x or !x
    Call = 6
}

lazy_static::lazy_static! {
static ref PRECEDENCES: std::collections::HashMap<token::TokenType, Precedence> = {
    let mut map = std::collections::HashMap::new();
    map.insert(token::TokenType::Equal,     Precedence::Equals);
    map.insert(token::TokenType::NotEqual,  Precedence::Equals);
    map.insert(token::TokenType::Lt,        Precedence::LessGreater);
    map.insert(token::TokenType::Gt,        Precedence::LessGreater);
    map.insert(token::TokenType::Plus,      Precedence::Sum);
    map.insert(token::TokenType::Minus,     Precedence::Sum);
    map.insert(token::TokenType::Slash,     Precedence::Product);
    map.insert(token::TokenType::Asterix,   Precedence::Product);
    
    map
    };
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    length: usize,
    ch: char

}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_valid_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn skip_whitespace(lex: &mut Lexer) {
    while is_whitespace(lex.ch) {
        lex.read_char();
    }
}

fn lookup_keyword(keyword: &String) -> token::TokenType {
    match KEYWORDS.get(&keyword[..]) {
        Some(key) => {
            //println!("deb key word: {}", keyword);
            key.clone()
        },
        None => {
            //println!("deb indentifier: {}", keyword);
            token::TokenType::Identifier
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let len = input.chars().count();
        let mut lex = Lexer {
            input,
            pos: 0,
            read_pos: 0,
            length: len,
            ch: char::default()
        };

        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.length {
            self.ch = char::default();
        } else {
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_pos < self.length {
            self.input.chars().nth(self.read_pos).unwrap()
        } else {
            char::default()
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while is_valid_identifier(self.ch) {
            self.read_char();
        }

        self.input[pos..self.pos+1].to_string() 
    }

    fn read_number(&mut self) -> String {
        let pos = self.pos;
        while is_digit(self.peek_char()) {
            self.read_char();
        }

        self.input[pos..self.pos+1].to_string()
    }

    pub fn next_token(&mut self) -> token::Token {
        skip_whitespace(self);
        let mut lit: String = self.ch.to_string();
        
        let tok = match self.ch {
            token::ASSIGN => {
                if self.peek_char() == token::ASSIGN {
                    self.read_char();
                    lit = String::from(token::EQUAL);
                    token::TokenType::Equal
                } else {
                    token::TokenType::Assign
                }
            },
            token::BANG => {
                if self.peek_char() == token::ASSIGN {
                    self.read_char();
                    lit = String::from(token::NOT_EQUAL);
                    token::TokenType::NotEqual
                } else {
                    token::TokenType::Bang
                }
            },
            token::PLUS => token::TokenType::Plus,
            token::MINUS => token::TokenType::Minus,
            token::SLASH => token::TokenType::Slash,
            token::ASTERIX => token::TokenType::Asterix,
            token::LT => token::TokenType::Lt,
            token::GT => token::TokenType::Gt,
            
            token::COMMA => token::TokenType::Comma,
            token::COLON => token::TokenType::Colon,
            token::SEMICOLON => token::TokenType::Semicolon,
            token::LPAREN => token::TokenType::Lparen,
            token::RPAREN => token::TokenType::Rparen,
            token::LBRACE => token::TokenType::Lbrace,
            token::RBRACE => token::TokenType::Rbrace,
            token::LBRACKET => token::TokenType::Lbracket,
            token::RBRACKET => token::TokenType::Rbracket,
            token::EOF => token::TokenType::Eof,
            _ => {
                //println!("deb {}", self.ch);
                if is_valid_identifier(self.ch) {
                    lit = self.read_identifier();
                    lookup_keyword(&lit)
                } else if is_digit(self.ch) {
                    lit = self.read_number();
                    token::TokenType::Integer
                } else {
                    token::TokenType::Invalid
                }
            }
        };
        
        let token = token::Token{token_type: tok, literal: lit};
        self.read_char();
        return token;
    }

    
}