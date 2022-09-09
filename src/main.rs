#[allow(dead_code)]
mod token;
#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod ast;
#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod test_lexer;
#[allow(dead_code)]
mod repl;
#[allow(dead_code)]
mod test_parser;

fn main() {
    // test_lexer::test_lexer();
    test_parser::test_parser();

    // repl::start();
}
