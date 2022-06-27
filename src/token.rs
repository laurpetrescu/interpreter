
pub const FUNCTION: &str	= "fn";
pub const LET: &str 		= "let";
pub const TRUE: &str 		= "true";
pub const FALSE: &str 		= "false";
pub const IF: &str	 		= "if";
pub const ELSE: &str 		= "else";
pub const RETURN: &str 		= "return";
pub const KAKA: &str 		= "kaka";
pub const MACA: &str 		= "maca";
 
pub const EQUAL: &str		= "==";
pub const NOT_EQUAL: &str	= "!=";

pub const EOF: char			= '\0';
pub const ASSIGN: char 		= '=';
pub const PLUS: char 		= '+';
pub const MINUS: char 		= '-';
pub const BANG: char 		= '!';
pub const ASTERIX: char		= '*';
pub const SLASH: char 		= '/';
pub const LT: char 			= '<';
pub const GT: char	 		= '>';

pub const COMMA: char 		= ',';
pub const SEMICOLON: char	= ';';
pub const COLON: char	    = ':';
pub const LPAREN: char 		= '(';
pub const RPAREN: char 		= ')';
pub const LBRACE: char 		= '{';
pub const RBRACE: char 		= '}';
pub const LBRACKET: char 	= '[';
pub const RBRACKET: char 	= ']';

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
	Invalid,
    Eof,
    Kaka,
    Maca,
    
    Identifier,
    Integer,

    Equal,
    NotEqual,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterix,
    Slash,
    Lt,
    Gt,
    
    Comma,
    Semicolon,
    Colon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

