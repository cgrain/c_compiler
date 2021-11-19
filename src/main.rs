mod lexer;
mod parser;
mod emitter;
// This file tests basic testing and ensures that all files are included in the build. This makes sure that everything is always tested.

fn main() {
    println!("Hello, world!");
    println!("Hi");
    let tokens = crate::lexer::lex_string("int main() { return 1; }");
    let _parsed = crate::parser::parse(&tokens);
}

