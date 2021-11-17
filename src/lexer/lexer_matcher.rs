use super::lexer_types::*;

pub fn is_token2(s: &str) -> bool {
    return is_keyword(s) || is_ident_2(s) || is_literal_decimal(s) || is_comment(s)
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


fn is_ident_2(s: &str) -> bool {
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


pub fn get_token2(s: &str) -> Token {
    if is_ident_2(s) {
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
