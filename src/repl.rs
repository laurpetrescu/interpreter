// use crate::lexer;
// use std::io::{Write};


// const PROMPT: &str = "lang>> ";

// pub fn start() {
    
//     loop {
//         println!("{}", PROMPT);
//         let mut line = String::new();

//         std::io::stdout().flush().expect("error while flushing");
//         std::io::stdin().read_line(&mut line).expect("error while getting string");
        
//         let mut lx = lexer::Lexer::new(line);
//         let tok = lx.next_token();
//         while  != token::TokenType::Eof {
//             println!("{:?}", to)
//         } 
//     }
// }