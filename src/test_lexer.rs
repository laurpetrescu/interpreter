use crate::lexer;
use crate::token;

pub fn test_lexer() {
    test_numbers();
    test_valid_operators();
    test_identifiers();
    test_keywords();

}

fn test_numbers() -> bool {
    let mut lx = lexer::Lexer::new("123 44 77 ".to_string());
    let mut tok = lx.next_token();
    
    print!("Testing integers: ");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type != token::TokenType::Integer {
            println!("failed {:?}", tok);
            return false;
        }

        tok = lx.next_token();
    }

    println!("success");
    true
}

fn test_valid_operators() -> bool {
    let mut lx = lexer::Lexer::new(" == != = + - ! * / < > , . ; : ( ) { } [ ] ".to_string());
    let mut tok = lx.next_token();

    print!("Testing operators: ");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type == token::TokenType::Invalid {
            println!("failed {:?}", tok);
            return false;
        }

        tok = lx.next_token();
    }

    println!("success");
    true
}

fn test_identifiers() -> bool {
    let mut lx = lexer::Lexer::new(" a a_a cucubau kakamaka YELL CaPSLock waterFuck".to_string());
    let mut tok = lx.next_token();

    print!("Testing identifiers: ");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type != token::TokenType::Identifier {
            println!("Test identifier failed {:?}", tok);
            return false;
        }
        tok = lx.next_token();
    }

    println!("success");
    true
}

fn test_keywords() -> bool {
    
    let mut lx = lexer::Lexer::new("fn let true false if else return while kaka maca".to_string());
    let mut tok = lx.next_token();

    print!("Testing keywords: ");
    while tok.token_type != token::TokenType::Eof {
        if tok.token_type == token::TokenType::Invalid {
            println!("Test keywords failed {:?}", tok);
            return false;
        }

        tok = lx.next_token();
    }

    println!("success");
    true
}
