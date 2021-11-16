pub fn return_5() -> i32 {
    println!("I am the Lexer");
    5
}


#[derive(PartialEq)]
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
    WhiteSpace,
}

#[derive(PartialEq)]
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
    println!("STARTING IN RECURSIVE");
    for c in s.chars() {
        s2.push(c);
        println!("{}", s2);
        if is_whitespace(&s2) { 
            let s_to_go = s.replacen(&s2, "",1);
            lex_string_recursive(&s_to_go, tokens);
            break;
        }
        if ! is_valid_token(&s2) {
            if s2.chars().count() == 1 { // WhiteSpace token, most likely. It can also be an UTF-8 Token
                
            }
            s2.pop();
            println!("{}", s2);
            println!("COMPLETE STRING: {}", s);
            let token = get_token(&s2);
            let s_to_go = s.replacen(&s2, "",1);
            if token.token_type != TokenType::WhiteSpace { 
                tokens.push(token);
            }
            lex_string_recursive(&s_to_go, tokens);
            s2 = "".to_string();
            break;
        }
    }
    println!("ENDING LOOP");
    let final_token = get_token(&s2);
    if final_token.token_type != TokenType::WhiteSpace { 
        tokens.push(final_token);
    }
    // return Token { token_type: TokenType::OpenBrace }
}

fn is_valid_token(s: &str) -> bool {
    if is_valid_identifier(s) { 
        return true;
    }
    if is_a_keyword(s) { 
        return false;
    }
    match s { 
        " " => return false,
        _ => return true,
    }
}
fn is_valid_identifier(s: &str) -> bool { 
    if is_a_keyword(s) { 
        return false; 
    }

    println!("{}", s);
    match s { 
        "{" => return false,
        " " => return false,
        _ => return true, 
    }
}
fn is_whitespace(s: &str) -> bool { 
    match s { 
        " " => return true, 
        _ => return false, 
    }
}
fn get_token(s: &str) -> Token {
    return Token {
        token_type: TokenType::OpenBrace,
    };
}
fn is_a_keyword(s: &str) -> bool {
    match s {
        "return " => true,
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
