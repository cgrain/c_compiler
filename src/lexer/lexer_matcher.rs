use super::lexer_types::*;


pub fn is_valid_token(s: &str) -> bool {
    if is_valid_identifier(s) {
        return true;
    }
    if is_a_keyword(s) {
        return true;
    }
    match s {
        "{" => return true,
        "}" => return true,
        "(" => return true,
        ")" => return true,
        ";" => return true,
//        "[" => return true,
//        "]" => return true,
//        "," => return true,
//        "." => return true,
        " " => return false,
        _ => return false,
    }
}
pub fn is_valid_identifier(s: &str) -> bool {
    if is_a_keyword(s) {
        return false;
    }
    for c in s.chars() {
        if !is_valid_ident_char(c) {
            println!("WHADDUP");
            return false;
        }
    }
    println!("{}", s);
    match s {
        "{" => return false,
        " " => return false,
        _ => return true,
    }
}


pub fn is_valid_ident_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A' ..='Z' | '0'..='9' => return true,
        _ => return false,
    }
}


pub fn is_whitespace(s: &str) -> bool {
    match s {
        " " => return true,
        _ => return false,
    }
}

pub fn get_token_type(s: &str) -> TokenType { 
    match s { 
        "{" => return TokenType::OpenBrace,
        "}" => return TokenType::CloseBrace,
        "(" => return TokenType::OpenParenthesis,
        ")" => return TokenType::CloseParenthesis,
        ";" => return TokenType::SemiColon,
        _ => panic!("Invalid token"),
    }
}


pub fn get_token(s: &str) -> Token {
    if is_a_keyword(s) { 
        return get_keyword(s);

    }
    if is_valid_identifier(s) { 
        return Token { 
            token_type: TokenType::Identifier,
        };
    }
    return Token {
        token_type: get_token_type(s),
    };
}


pub fn is_a_keyword(s: &str) -> bool {
    match s {
        "return" => true,
        "int" => true,
        _ => false,
    }
}

pub fn get_keyword_type(s: &str) -> TokenType { 
    match s { 
        "return" => return TokenType::Return,
        "int" => return TokenType::Int,
        _ => panic!("Invalid keyword"),
    }
}
pub fn get_keyword(s: &str) -> Token { 
    return Token { 
        token_type: get_keyword_type(s),
    };
}

#[cfg(test)]
mod valid_keyword {
    use super::*;
    #[test]
    fn return_is_a_keyword() {
        assert_eq!(is_a_keyword("return"), true)
    }
    #[test]
    fn test_is_not_a_keyword() {
        assert_eq!(is_a_keyword("test"), false)
    }
}