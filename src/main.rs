mod lexer;

// This file tests basic testing and ensures that all files are included in the build. This makes sure that everything is always tested. 

fn main() {
    println!("Hello, world!");
    println!("Hi");
    let b = crate::compiler::return_2();
    println!("{}", b);
}










#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn return_2_returns2() { 
        assert_eq!(crate::compiler::return_2(), 2)
    }
}
The following tokens are recognized by the preprocessor when they are used outside the context of a preprocessor directive:


