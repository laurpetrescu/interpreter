use crate::lexer;
use crate::parser;
use crate::ast;

pub fn test_parser() {
    // test_let_statement();
    // test_return_statement();
    test_expression_statement();

    // test_program_to_string();
}

fn test_program_to_string() -> bool {
    print!("Testing program to string: ");
    let input = "let x = 55;";
    let mut parser = parser::Parser::new(lexer::Lexer::new(input.to_string()));
    
    match parser.parse_program() {
        Ok(p) => println!("success - program string: {}", ast::Node::to_string(&p)),
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
    
    true
}

fn test_let_statement() -> bool {
    print!("Testing let statement: ");
    let input = "let x = 5;";
    let mut parser = parser::Parser::new(lexer::Lexer::new(input.to_string()));
    
    match parser.parse_program() {
        Ok(_) => println!("success"),
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
    
    true
}

fn test_return_statement() -> bool {
    print!("Testing return statement: ");
    let input = "return 5;";
    let mut parser = parser::Parser::new(lexer::Lexer::new(input.to_string()));
    
    match parser.parse_program() {
        Ok(_) => println!("success"),
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
    
    true
}

fn test_expression_statement() -> bool {
    print!("Testing expression statement: ");
    let input = "let x = 5 + 6 + 10;";
    let mut parser = parser::Parser::new(lexer::Lexer::new(input.to_string()));
    
    match parser.parse_program() {
        Ok(p) => println!("success {}", ast::Node::to_string(&p)),
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
    
    true
}