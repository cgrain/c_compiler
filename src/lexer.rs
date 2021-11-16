pub fn return_5()-> i32 { 
    println!("I am the Lexer");
    5
}
#[cfg(test)]
mod tests {
    #[test]
    fn return_5_returns5() { 
        assert_eq!(crate::lexer::return_5(), 5)
    }
}