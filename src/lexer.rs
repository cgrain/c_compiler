pub fn return_5()-> i32 { 
    println!("I am the Lexer");
    5
}

pub enum TokenType { 
    OpenBrace,
    CloseBrace, 
    OpenParenthesis,
    CloseParenthesis,
    SemiColon, 
    Int,
    Return,
    Identifier,
    IntegerLiteral

}


pub struct Token { 
    token_type: TokenType 
}


pub fn lex_string() -> crate::lexer::Token { 

    return Token { token_type: TokenType::OpenBrace }
}


#[cfg(test)]
mod tests {
    #[test]
    fn return_5_returns5() { 
        assert_eq!(crate::lexer::return_5(), 5)
    }
}