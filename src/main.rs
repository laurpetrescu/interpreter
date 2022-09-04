#[allow(dead_code)]
mod token;
#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod test_lexer;

use std::io::{Write};

fn main() {
    println!("Testing Lexer");

    test_lexer::test_numbers();
    test_lexer::test_valid_operators();
    test_lexer::test_identifiers();
    test_lexer::test_keywords();

    // let s = "fn f(i32 a): i32 {
    //     return a + 10;
    // }
    // let a = i32[2];
    // print(a[0]);";

    
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
