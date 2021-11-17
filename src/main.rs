mod lexer;
mod parser;
// This file tests basic testing and ensures that all files are included in the build. This makes sure that everything is always tested.

fn main() {
    println!("Hello, world!");
    println!("Hi");
    let _c = crate::lexer::lex_string("return Hello return World");
}

