mod lexer;
mod parser;
mod emitter;
use std::env;
use std::fs;
use std::process::Command;
// This file tests basic testing and ensures that all files are included in the build. This makes sure that everything is always tested.


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let asm = compile_file(filename);
    let file_header = filename.split('.').next().unwrap();
    let file_path = format!("{}.s", file_header);
    fs::write(&file_path, asm).expect("Unable to write file");
    Command::new("gcc").arg(file_path)
    .arg("-o").arg("out")
    .spawn().expect("Unable to run gcc");
    println!("SUCCESFULLY BUILD FILE")
}


fn compile(input: &str) -> String {
    let tokens = crate::lexer::lex_string(input);
    let parsed = crate::parser::parse(&tokens);
    if parsed == None {
        panic!("Parsing failed");
    }
    let parsed = parsed.unwrap();
    println!("PARSED NODES: {:?}", parsed);
    let emitted = crate::emitter::emit(&parsed);
    return emitted;
}

fn compile_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    return compile(&contents);
}


#[cfg(test)]
mod tests { 
    use super::*;
    
    #[test]
    fn compile_no_enters() {
        let asm = compile("int main() { return 1; }");
        println!("{}", asm);
        let answer_asm = "    .globl main\n main:\n     mov $1, %rax \n    ret\n";
        assert_eq!(asm, answer_asm);
    }
    #[test]
    fn compile_return_42() { 
        let asm = compile("int main() { return 42; }");
        println!("{}", asm);
        let answer_asm = "    .globl main\n main:\n     mov $42, %rax \n    ret\n";
        assert_eq!(asm, answer_asm);
    }
    #[test]
    fn compile_test_file() {
        let asm = compile_file("test_files/return_2.c");
        println!("{}", asm);
        let answer_asm = "    .globl main\n main:\n     mov $2, %rax \n    ret\n";
        assert_eq!(asm, answer_asm);
    }
}