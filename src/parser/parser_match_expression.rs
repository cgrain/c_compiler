use super::parser_nodes::*; 
use crate::lexer::lexer_types::*;

pub fn parse_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Expression> := int
    return parse_primary_expression(token_list);
}

fn parse_primary_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) {
        Some( Token { token_type: TokenType::IntegerLiteral, value: v, ..}) => {
            return (Some(Node { kind: NodeKind::Expression, parent:None, children: vec![], value:v.clone() }), &token_list[1..]);
        }
        Some( Token { token_type: TokenType::Identifier, name: n, ..}) => {
            unimplemented!();
        }
        _ => {
            return (None, token_list);
        }
    }
} 

fn parse_expression_in_brackets(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Expression> := OpenParenthesis <Expression> CloseParenthesis
    match token_list.get(0) {
        Some( Token { token_type: TokenType::OpenParenthesis, ..}) => {
            // We know this matches (Otherwise it would not be called), but just to be sure
            let mut open_parenthesis = 1; 
            for i in 1..token_list.len() {
                if token_list[i].token_type == TokenType::CloseParenthesis { 
                    open_parenthesis -= 1;
                    if open_parenthesis == 0 {
                        let (expression, rest) = parse_expression(&token_list[1..i]);
                        return (expression, &token_list[i+1..]);
                    }
                } else if token_list[i].token_type == TokenType::OpenParenthesis {
                    open_parenthesis += 1;
                }

            }
            panic!("Unbalanced Parenthesis!")
        } 
        _ => {
            return (None, token_list);
        }
    }
}




pub fn parse_number_literal(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) {
        Some(Token {
            token_type: TokenType::IntegerLiteral,
            value: value,
            .. } ) => {
                return (Some (Node { 
                    kind: NodeKind::Literal,
                    parent: None,
                    children: vec![],
                    value: value.clone(),
                }), &token_list[1..]);
            } 
        _ => {
            return (None, token_list);
        }
    }
} 


pub fn parse_unary_operator(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) { 
        Some ( Token { token_type: tt, ..}) => {
            match tt { 
                TokenType::Minus => {
                    unimplemented!();
                }, 
                TokenType::BitwiseNot => {
                    unimplemented!();
                },
                TokenType::Not => {
                    unimplemented!();
                }, 
                TokenType::Star => {
                    unimplemented!();
                },
                TokenType::Ampersand => {
                    unimplemented!();
                },
                TokenType::Plus => {
                    unimplemented!();
                },
                TokenType::Increment => {
                    unimplemented!();
                },
                TokenType::Decrement => {
                    unimplemented!();
                },
                _ => { return (None, token_list); }
            }
        }
        _ => return (None, token_list)
    }
}
