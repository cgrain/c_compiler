use super::parser_nodes::*;
use crate::lexer::lexer_types::*;

pub fn parse_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Expression> := int
    return parse_unary_operator(token_list);
}

fn parse_primary_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    let node_primary: Option<Node>;
    let return_list: &[Token];
    match token_list.get(0) {
        Some(Token {
            token_type: TokenType::IntegerLiteral,
            value: v,
            ..
        }) => {
            node_primary = Some(Node {
                kind: NodeKind::Expression,
                parent: None,
                children: vec![],
                value: v.clone(),
            });
            return_list = &token_list[1..];
        }
        Some(Token {
            token_type: TokenType::Identifier,
            name: n,
            ..
        }) => {
            unimplemented!();
        }
        Some(Token {
            token_type: TokenType::OpenParenthesis,
            ..
        }) => {
            return parse_expression_in_brackets(token_list);
        }
        Some(Token {
            token_type: TokenType::StringLiteral,
            value: v,
            ..
        }) => {
            unimplemented!();
        }
        Some(Token {
            token_type: TokenType::Keyword,
            name: n,
            ..
        }) => {
            let generic = "_Generic".to_string();
            let keyword = n.clone().unwrap();
            if *keyword == generic {
                unimplemented!();
            }
            return (None, token_list);
        }
        _ => {
            return (None, token_list);
        }
    }
    let (post, rest) = parse_postfix_expression(return_list);
    if post.is_some() {
        let mut node_post = post.unwrap();
        node_post.children.push(node_primary.unwrap());
        return (Some(node_post), rest);
    } else {
        return (node_primary, rest);
    }
}

fn parse_expression_in_brackets(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Expression> := OpenParenthesis <Expression> CloseParenthesis
    match token_list.get(0) {
        Some(Token {
            token_type: TokenType::OpenParenthesis,
            ..
        }) => {
            // We know this matches (Otherwise it would not be called), but just to be sure
            let mut open_parenthesis = 1;
            for i in 1..token_list.len() {
                if token_list[i].token_type == TokenType::CloseParenthesis {
                    open_parenthesis -= 1;
                    if open_parenthesis == 0 {
                        let (expression, rest) = parse_expression(&token_list[1..i]);
                        return (expression, &token_list[i + 1..]);
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
            ..
        }) => {
            return (
                Some(Node {
                    kind: NodeKind::Literal,
                    parent: None,
                    children: vec![],
                    value: value.clone(),
                }),
                &token_list[1..],
            );
        }
        _ => {
            return (None, token_list);
        }
    }
}

#[allow(unused)]
fn parse_string_literal(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    unimplemented!();
}

fn parse_identifier(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) {
        Some(Token {
            token_type: TokenType::Identifier,
            name: n,
            ..
        }) => {
            return (
                Some(Node {
                    kind: NodeKind::Identifier,
                    parent: None,
                    children: vec![],
                    value: n.clone(),
                }),
                &token_list[1..],
            );
        }
        _ => {
            return (None, token_list);
        }
    }
}

fn parse_postfix_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // Gets called by Primary Expression, because right-associative.
    return (None, token_list);
}



pub fn parse_unary_operator(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) {
        Some(Token { token_type: tt, .. }) => {
        let primary_expressions = vec![ 
            TokenType::IntegerLiteral, 
            TokenType::Identifier,
            TokenType::OpenParenthesis,
            TokenType::StringLiteral,
            TokenType::Keyword,
        ];
        if primary_expressions.contains(&tt) {
            return parse_primary_expression(token_list)
        }
        
        match tt {
            TokenType::Minus => {
                let (node, rest) = parse_expression(&token_list[1..]);
                return (
                    Some(Node {
                        kind: NodeKind::UnaryOperator,
                        parent: None,
                        children: vec![node.unwrap()],
                        value: Some("-".to_string()),
                    }),
                    rest,
                );
                unimplemented!();
            }
            TokenType::BitwiseNot => {
                let (node, rest) = parse_expression(&token_list[1..]);
                return (
                    Some(Node {
                        kind: NodeKind::UnaryOperator,
                        parent: None,
                        children: vec![node.unwrap()],
                        value: Some("~".to_string()),
                    }),
                    rest,
                );
                unimplemented!();
            }
            TokenType::Not => {
                let (node, rest) = parse_expression(&token_list[1..]);
                return (
                    Some(Node {
                        kind: NodeKind::UnaryOperator,
                        parent: None,
                        children: vec![node.unwrap()],
                        value: Some("!".to_string()),
                    }),
                    rest,
                );
                unimplemented!();
            }
            TokenType::Star => {
                unimplemented!();
            }
            TokenType::Ampersand => {
                unimplemented!();
            }
            TokenType::Plus => {
                unimplemented!();
            }
            TokenType::Increment => {
                unimplemented!();
            }
            TokenType::Decrement => {
                unimplemented!();
            }
            _ => {
                return (None, token_list);
            }

        }
    },

        _ => return (None, token_list),
    }
}
