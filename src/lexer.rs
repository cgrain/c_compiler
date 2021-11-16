pub fn return_5() -> i32 {
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
    IntegerLiteral,
}

pub struct Token {
    token_type: TokenType,
}
pub fn lex_string(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    lex_string_recursive(s, &mut tokens);
    return tokens;
}

pub fn lex_string_recursive(s: &str, tokens: &mut Vec<Token>) {
    let mut s2 = String::new();
    for c in s.chars() {
        s2.push(c);
        if is_valid_token(&s2) {
            println!("{}", s2);
            let token = get_token(&s2);
            let s_to_go = s.replace(&s2, "");
            tokens.push(token);
            lex_string_recursive(&s_to_go, tokens);
        }
    }
    // return Token { token_type: TokenType::OpenBrace }
}

fn is_valid_token(s: &str) -> bool {
    return true;
}
fn get_token(s: &str) -> Token {
    return Token {
        token_type: TokenType::OpenBrace,
    };
}
fn is_a_keyword(s: &str) -> bool {
    match s {
        "return" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
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
}
