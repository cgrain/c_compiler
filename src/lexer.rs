mod lexer_matcher;
mod lexer_types;
use lexer_matcher::*;
use lexer_types::*;


pub fn lex_string(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut s2 = s.to_string().clone();
    while s2 != "".to_string() {
        let token: Token;
        let res = lex_string_inner(&s2);
        match res {
            (t, s) => {
                token = t;
                s2 = s;
            }
        }
        if token.token_type != TokenType::WhiteSpace {
            tokens.push(token);
        }
    }
    return tokens;
}

pub fn lex_string_inner(s: &str) -> (Token, String) {
    let mut s2 = String::new();
    for c in s.chars() {
        s2.push(c);
        if lexer_matcher::is_whitespace(&s2) {
            let s_to_go = s.replacen(&s2, "", 1);
            return (
                Token {
                    token_type: TokenType::WhiteSpace,
                    value: None,
                    name: None,
                },
                s_to_go,
            );
        }
        if !is_token(&s2) {
            s2.pop();
            let token = get_token(&s2);
            let s_return = s.replacen(&s2, "", 1);
            return (token, s_return);
        }
    }

    return (get_token(&s2), "".to_string());
}






#[cfg(test)]
mod tests {
    use crate::lexer::*;
    
    mod testing_match_workings { 
        #[test]
        fn test_match_basics() { 
            assert_eq!(1,1);
        }
    }
    mod test_basic_example { 
        use super::*;
        #[test]
        fn test_basic_statement() { 
            let s = "return 1;"; 
            let tokens = lex_string(s);
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].token_type, TokenType::Return);
            assert_eq!(tokens[1].token_type, TokenType::IntegerLiteral);
            assert_eq!(tokens[2].token_type, TokenType::Semicolon);
        }
    }
}
