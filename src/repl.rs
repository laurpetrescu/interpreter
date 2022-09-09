use crate::lexer;
use crate::token;
use std::io::{Write};

pub fn start() {
    loop {
        print!(">> ");
        let mut line = String::new();
        std::io::stdout().flush().expect("error while flushing");
        std::io::stdin().read_line(&mut line).expect("error while getting string");
        
        let mut lx = lexer::Lexer::new(line);
        let mut tok = lx.next_token();
        print!("tok {}", tok.literal);
        if &tok.literal == "exit" {
            break;
        }
        while tok.token_type != token::TokenType::Eof {
            println!("{:?}", tok);
            tok = lx.next_token();
        }
        
    }
}
