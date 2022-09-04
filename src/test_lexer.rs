use crate::lexer;
use crate::token;

// pub fn test_next_token(input_string: String) {
//     let mut lx = Lexer::new(input_string);
//     let mut tok = lx.next_token();
//     while tok.token_type != token::TokenType::Eof {
//         println!("{:?}", tok);
//         tok = lx.next_token();
//     }
// }

pub fn test_numbers() -> bool {
    let mut lx = lexer::Lexer::new("123 44 77 ".to_string());
    let mut tok = lx.next_token();
    
    println!("Testing integers");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type != token::TokenType::Integer {
            println!("Test integers failed");
            return false;
        }

        tok = lx.next_token();
    }

    true
}

pub fn test_valid_operators() -> bool {
    let mut lx = lexer::Lexer::new(" == != = + - ! * / < > , . ; : ( ) { } [ ] ".to_string());
    let mut tok = lx.next_token();

    println!("Testing operators");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type == token::TokenType::Invalid {
            println!("Test operators failed {:?}", tok);
            return false;
        }

        tok = lx.next_token();
    }

    true
}

pub fn test_identifiers() -> bool {
    let mut lx = lexer::Lexer::new(" a a_a cucubau kakamaka YELL CaPSLock waterFuck".to_string());
    let mut tok = lx.next_token();

    println!("Testing identifiers");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type != token::TokenType::Identifier {
            println!("Test identifier failed {:?}", tok);
            return false;
        }
        tok = lx.next_token();
    }

    true
}

pub fn test_keywords() -> bool {
    
    let mut lx = lexer::Lexer::new("fn let true false if else return while kaka maca".to_string());
    let mut tok = lx.next_token();

    println!("Testing keywords");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type == token::TokenType::Invalid {
            println!("Test keywords failed {:?}", tok);
            return false;
        }

        tok = lx.next_token();
    }

    true
}