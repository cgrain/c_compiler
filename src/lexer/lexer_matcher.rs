use super::lexer_types::*;

pub fn is_token(s: &str) -> bool {
    return is_keyword(s) || is_ident(s) || is_literal_decimal(s) || is_comment(s)
}


pub fn is_keyword(s: &str) -> bool {
    match s {
        "return" => return true,
        "int" => return true,
        "{" => return true,
        "}" => return true,
        "(" => return true,
        ")" => return true,
        ";" => return true,
        //        "[" => return true,
        //        "]" => return true,
        //        "," => return true,
        //        "." => return true,
        _ => return false,
    }
} 


fn is_ident(s: &str) -> bool {
    if is_keyword(s) {
        return false;
    }
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => continue,
            _ => return false,
        }
    }
    let c0 = s.chars().next();
    match c0 {
        Some('a'..='z' | 'A'..='Z') => return true,
        _ => return false,
    }
}


fn is_literal_decimal(s: &str) -> bool {
    let mut decimal_points: i8 = 0;
    for c in s.chars() {
        match c {
            '0'..='9' => continue,
            '.' => {
                decimal_points += 1;
                if decimal_points > 1 {
                    return false;
                }
                continue;
            }
            _ => return false,
        }
    }
    return true;
}


fn is_comment(_s: &str) -> bool {
    false
}


pub fn get_token(s: &str) -> Token {
    if is_ident(s) {
        // return Token::Ident(s.to_string());
        return Token { token_type: TokenType::Identifier, name: Some(s.to_string()), value: None };
    }
    if is_literal_decimal(s) {
        return Token { token_type: TokenType::IntegerLiteral, name: None, value: Some(s.to_string()) };
    }
    if is_keyword(s) {
        return Token { token_type: get_keyword_token_type(s), name: None, value: None };
    }
   if is_comment(s) {
        return Token { token_type: TokenType::WhiteSpace, name: None, value: None };
    }
    panic!("Unknown token: {}", s);
}


fn get_keyword_token_type(s: &str) -> TokenType  { 
    match s {
        "return" => return TokenType::Return,
        "int" => return TokenType::Int,
        "{" => return TokenType::OpenBrace,
        "}" => return TokenType::CloseBrace,
        "(" => return TokenType::OpenParenthesis,
        ")" => return TokenType::CloseParenthesis,
        ";" => return TokenType::Semicolon,
        //        "[" => return true,
        //        "]" => return true,
        //        "," => return true,
        //        "." => return true,
        _ => panic!("Invalid Token") 
    }


}


pub fn is_whitespace(s: &str) -> bool {
    match s {
        " " => return true,
        _ => return false,
    }
}


#[cfg(test)]
mod valid_keyword {
    use super::*;
    #[test]
    fn test_return() {
        assert_eq!(is_keyword("return"), true);
    }
    #[test]
    fn test_int() {
        assert_eq!(is_keyword("int"), true);
    }
    #[test]
    fn test_open_brace() {
        assert_eq!(is_keyword("{"), true);
    }
    #[test]
    fn test_close_brace() {
        assert_eq!(is_keyword("}"), true);
    }
    #[test]
    fn test_open_parenthesis() {
        assert_eq!(is_keyword("("), true);
    }
    #[test]
    fn test_close_parenthesis() {
        assert_eq!(is_keyword(")"), true);
    }
    #[test]
    fn test_semicolon() {
        assert_eq!(is_keyword(";"), true);
    }
} 
#[cfg(test)]
mod invalid_keyword {
    use super::*;
    #[test]
    fn test_ident() {
        assert_eq!(is_keyword("ident"), false);
    }
    #[test]
    fn test_decimal() {
        assert_eq!(is_keyword("decimal"), false);
    }
    #[test]
    fn test_whitespace() {
        assert_eq!(is_keyword(" "), false);
    }
}
#[cfg(test)]
mod valid_ident {
    use super::*;
    #[test]
    #[allow(non_snake_case)]
    fn test_azAZ() { 
        let test = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert_eq!(is_ident(test), true);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_0_9() { 
        let test = "a0123456789";
        assert_eq!(is_ident(test), true);
    }

}
mod valid_decimal {
    use super::*;
    #[test]
    fn test_0_9() { 
        let test = "0.123456789";
        assert_eq!(is_literal_decimal(test), true);
    }
    #[test]
    fn test_two_dots() { 
        let test = "0.123456789.";
        assert_eq!(is_literal_decimal(test), false);
    }
    #[test]
    fn no_letters() { 
        let test = "0.123456789a";
        assert_eq!(is_literal_decimal(test), false);
    }
}

#[cfg(test)]
mod valid_whitespace {
    use super::*;
    #[test]
    fn test_whitespace() { 
        let test = " ";
        assert_eq!(is_whitespace(test), true);
    }
}
#[cfg(test)]
mod invalid_whitespace {
    use super::*;
    #[test]
    fn test_whitespace() { 
        let test = "a";
        assert_eq!(is_whitespace(test), false);
    }
}
#[cfg(test)]
mod get_ident_token {
    use super::*;
    #[test]
    fn test_ident() {
        let test = "ident";
        assert_eq!(get_token(test), Token { token_type: TokenType::Identifier, name: Some("ident".to_string()), value: None });
    }
}
#[cfg(test)]
mod get_decimal_token {
    use super::*;
    #[test]
    fn test_decimal() {
        let test = "0.123456789";
        assert_eq!(get_token(test), Token { token_type: TokenType::IntegerLiteral, name: None, value: Some("0.123456789".to_string()) });
    }
}
#[cfg(test)]
mod get_keyword_token {
    use super::*;
    #[test]
    fn test_return() {
        let test = "return";
        assert_eq!(get_token(test), Token { token_type: TokenType::Return, name: None, value: None });
    }
    #[test]
    fn test_int() {
        let test = "int";
        assert_eq!(get_token(test), Token { token_type: TokenType::Int, name: None, value: None });
    }
    #[test]
    fn test_open_brace() {
        let test = "{";
        assert_eq!(get_token(test), Token { token_type: TokenType::OpenBrace, name: None, value: None });
    }
    #[test]
    fn test_close_brace() {
        let test = "}";
        assert_eq!(get_token(test), Token { token_type: TokenType::CloseBrace, name: None, value: None });
    }
    #[test]
    fn test_open_parenthesis() {
        let test = "(";
        assert_eq!(get_token(test), Token { token_type: TokenType::OpenParenthesis, name: None, value: None });
    }
    #[test]
    fn test_close_parenthesis() {
        let test = ")";
        assert_eq!(get_token(test), Token { token_type: TokenType::CloseParenthesis, name: None, value: None });
    }
    #[test]
    fn test_semicolon() {
        let test = ";";
        assert_eq!(get_token(test), Token { token_type: TokenType::Semicolon, name: None, value: None });
    }
}