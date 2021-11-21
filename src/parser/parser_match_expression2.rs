use crate::lexer::lexer_types::{Token, TokenType};
use super::parser_nodes::{Node, NodeKind};


pub fn operator_precedence(token: &Token) -> i32 {
    match token.token_type {
        TokenType::Star | TokenType::Slash | TokenType::Percent => 20-3,
        TokenType::Plus | TokenType::Minus => 20-4,
        _ => 0,
    }
}
pub fn parse_unary_operator(token_list: &[Token]) -> (Option<Node>, &[Token]) { 
    let node_primary: Option<Node>;
    let return_list: &[Token];
    match token_list.get(0) {
        Some(Token {
            token_type: TokenType::IntegerLiteral,
            value: v,
            ..
        }) => {
            node_primary = Some(Node {
                kind: NodeKind::Literal,
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
    return (node_primary, return_list);
}


pub fn parse_binary_operators(
    token_list: &[Token], 
    operator_level: i32, 
) -> (Option<Node>, &[Token])  { 
    println!("Parsing Binary Operand, OP_LEVEL: {}", operator_level);
    // <Binary Operand> ::= <Unary Operand> {<Binary Operator> <Unary Operand>}
    // <Binary Operator> ::= * | / | % | + | -
    let (mut left_side, mut rest) = parse_unary_operator(token_list);
    println!("tokens: \n\n{:?}\n", token_list);
    println!("Unary {:?}", left_side);
    if left_side == None {
        return (None, rest);
    }
    let mut counter = 0;
    loop { 
        let token = rest.get(0);
        println!("Token {}: {:?}", counter, token);
        if token == None || token.unwrap().token_type == TokenType::Semicolon {
            return (left_side, rest);
        }
        let token = token.unwrap();
        let operator_level_current = operator_precedence(token);
        println!("Operator Level: {}", operator_level_current);
        if operator_level_current < operator_level { 
            println!("Return because acceptable level");
            return (left_side, rest);
        }
        let new_level = operator_level_current + 1;
        let (rhs, rest_rhs) = parse_binary_operators(&rest[1..], new_level);
        if rhs == None { 
            panic!("Expected a RightHand Side with Operator {:?}", token);
        }
        rest = rest_rhs;
        println!("WE FOUND A NODE AND ARE GONIG TO CAPTURE IT");
        left_side = Some(Node {
            kind: NodeKind::BinaryOperator,
            parent: None,
            children: vec![left_side.unwrap(), rhs.unwrap()],
            value: token.value.clone(),
        });

        counter += 1;
        if counter > 100 {
            panic!("Infinite Loop Detected");
        }
    }
    // 5 + 5 + 5: 
    // (5 + 5) + 5:
    // ((5 + 5) + 5):

    // 5 + 5 x 5:
    // 5 + (5 x 5):
    // (5 + (5 x 5)):

    // 5 * 5 + 5: 
    // (5 * 5) + 5:
    // ((5 * 5) + 5):
}

#[cfg(test)]
mod binary_test { 
    use super::*;
    #[test]
    fn test_5_plus_5() { 
        let tokens = vec![
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Plus,
                value: Some("+".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Semicolon,
                value: None, 
                name: None, 
            }
        ];
        let (node, rest) = super::parse_binary_operators(&tokens, 0);
        assert_eq!(
            node.unwrap(), 
            Node { 
                kind: NodeKind::BinaryOperator,
                parent: None,
                children: vec![
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                ],
                value: Some("+".to_string()),
            }
        )
    }
    #[test]
    fn test_5_plus_5_plus_5() { 
        let tokens = vec![
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Plus,
                value: Some("+".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Plus,
                value: Some("+".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Semicolon,
                value: None, 
                name: None, 
            }
        ];
        let (node, rest) = super::parse_binary_operators(&tokens, 0);
        assert_eq!(
            node.unwrap(), 
            Node { 
                kind: NodeKind::BinaryOperator,
                parent: None,
                children: vec![
                    Node { 
                        kind: NodeKind::BinaryOperator,
                        parent: None,
                        children: vec![
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                        ],
                        value: Some("+".to_string()),
                    },
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                ],
                value: Some("+".to_string()),
            }
        )
    }
    #[test]
    fn test_5_times_5() { 
        let tokens = vec![
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Star,
                value: Some("*".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Semicolon,
                value: None, 
                name: None, 
            }
        ];
        let (node, rest) = super::parse_binary_operators(&tokens, 0);
        assert_eq!(
            node.unwrap(), 
            Node { 
                kind: NodeKind::BinaryOperator,
                parent: None,
                children: vec![
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                ],
                value: Some("*".to_string()),
            }
        )
    }
    #[test]
    fn test_5_plus_5_times_5() { 
        let tokens = vec![
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Plus,
                value: Some("+".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Star,
                value: Some("*".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Semicolon,
                value: None, 
                name: None, 
            }
        ];
        let (node, rest) = super::parse_binary_operators(&tokens, 0);
        assert_eq!(
            node.unwrap(), 
            Node { 
                kind: NodeKind::BinaryOperator,
                parent: None,
                children: vec![
                    
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                    Node { 
                        kind: NodeKind::BinaryOperator,
                        parent: None,
                        children: vec![
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                        ],
                        value: Some("*".to_string()),
                    },
                ],
                value: Some("+".to_string()),
            }
        )
    }
    #[test]
    fn test_5_times_5_plus_5() { 
        let tokens = vec![
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Star,
                value: Some("*".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::Plus,
                value: Some("+".to_string()),
                name: None,
            },
            Token { 
                token_type: TokenType::IntegerLiteral,
                value: Some("5".to_string()),
                name: None,
            },
        ];
        let (node, rest) = super::parse_binary_operators(&tokens, 0);
        assert_eq!(
            node.unwrap(), 
            Node { 
                kind: NodeKind::BinaryOperator,
                parent: None,
                children: vec![
                    Node { 
                        kind: NodeKind::BinaryOperator,
                        parent: None,
                        children: vec![
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                            Node { 
                                kind: NodeKind::Literal,
                                parent: None,
                                children: vec![],
                                value: Some("5".to_string()),
                            },
                        ],
                        value: Some("*".to_string()),
                    },
                    Node { 
                        kind: NodeKind::Literal,
                        parent: None,
                        children: vec![],
                        value: Some("5".to_string()),
                    },
                ],
                value: Some("+".to_string()),
            }
        )
    }
}