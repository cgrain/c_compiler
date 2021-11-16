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


pub fn lex_string_recursive(s: &str, tokens:&mut Vec<Token> ) { 
    let mut s2 = String::new();
    for c in s.chars() { 
        s2.push(c);
        if is_valid_token(&s2) { 
            println!("{}" , s2);
            let token = get_token(&s2);
            let s_to_go = s.replace(&s2, "");
            tokens.push(token);
            lex_string_recursive(&s_to_go, tokens);
        }
        

    }
    // return Token { token_type: TokenType::OpenBrace }
}
pub fn iterate_string(s: &str) { 
    let mut s2 = String::new();
    for c in s.chars() { 
        s2.push(c);
        println!("{}" , s2)
    }
}
fn is_valid_token(s: &str) -> bool { 
    return true
}
fn get_token(s: & str) -> Token { 
    return Token { token_type: TokenType::OpenBrace }
}
fn is_a_keyword(s: &str) -> bool { 
    match s { 
        "return" => true, 
        _ => false 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn return_5_returns5() { 
        assert_eq!(crate::lexer::return_5(), 5)
    }
    #[test]
    fn fail_iterate_string() { 
        crate::lexer::iterate_string("Hello World");
        assert_eq!(1,  2);
    }
    
}