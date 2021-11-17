use super::parser_nodes::*;
use crate::lexer::lexer_types::*;


fn match_token(token_list: &[Token], match_token_type: TokenType) -> bool {
    match token_list.get(0) {
        Some(Token { token_type: tt, .. }) => tt == &match_token_type,
        _ => return false,
    }
}


pub fn parse_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Expression> := int
    if match_token(token_list, TokenType::IntegerLiteral) { 
        return(
            Some(Node {
                kind: NodeKind::Expression,
                parent: None,
                children: vec![],
            }),
            &token_list[1..],
        );
    }
    else { 
        return (None, token_list);
    }
}
pub fn parse_statement(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    return parse_return_statement(token_list);
}

pub fn parse_return_statement(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <ReturnStatement> := <return> <Expression> <SemiColon>
    if match_token(token_list, TokenType::Return) {
        let (expression, token_list) = parse_expression(token_list);
        if expression == None {
            return (None, token_list);
        }
        if match_token(token_list, TokenType::Semicolon) {
            return (
                Some(Node {
                    kind: NodeKind::Statement,
                    parent: None,
                    children: vec![expression.unwrap()],
                }),
                &token_list[1..],
            );
        }
    }
    return (None, token_list);
}



pub fn parse_block(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    match token_list.get(0) {
        Some(token) => match token.token_type {
            TokenType::OpenBrace => {
                let mut children = vec![];
                let mut remainder = &token_list[1..];
                while let Some(token) = remainder.get(0) {
                    if token.token_type == TokenType::CloseBrace {
                        return (
                            Some(Node {
                                kind: NodeKind::Block,
                                parent: None,
                                children: children,
                            }),
                            &remainder[1..],
                        );
                    }
                    let (child, new_remainder) = parse_statement(remainder);
                    if child == None {
                        return (None, &token_list);
                    }
                    children.push(child.unwrap());
                    remainder = new_remainder;
                }
                return (None, &token_list);
            }
            _ => {
                return (None, &token_list);
            }
        },
        None => {
            return (None, &token_list);
        }
    }
}
pub fn parse_parameter_list(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // TODO
    return (None, &token_list);
}
pub fn parse_function_header(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <FunctionHeader> := <Identifier> <Identifier> <OpenParen> <ParameterList> <CloseParen> <Block>
    if let Some(token) = token_list.get(0) {
        if token.token_type != TokenType::Identifier {
            return (None, &token_list);
        }
    } else {
        return (None, &token_list);
    }
    let token1 = token_list.get(1);
    if token1 == None {
        return (None, &token_list);
    }
    let token1_wrap = token1.unwrap();
    if token1_wrap.token_type != TokenType::Identifier {
        return (None, &token_list);
    }
    let token1 = token_list.get(2);
    if token1 == None {
        return (None, &token_list);
    }
    let token1_wrap = token1.unwrap();
    if token1_wrap.token_type != TokenType::OpenParenthesis {
        return (None, &token_list);
    }

    let (node, remainder) = parse_parameter_list(&token_list[1..]);
    if node == None {
        return (None, &token_list);
    }
    let first_remainder = remainder.get(0);
    if first_remainder == None {
        return (None, &token_list);
    } else {
        return (None, &token_list);
    }
}
pub fn parse_function(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Function> := <FunctionHeader> <Block>

    todo!()
}
pub fn parse_class(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    todo!()
}
pub fn parse_program(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    todo!()
}

fn equal_nodes(node1: &Node, node2: &Node) -> bool {
    if node1.kind == node2.kind && node1.children.len() == node2.children.len() {
        for i in 0..node1.children.len() {
            if !equal_nodes(&node1.children[i], &node2.children[i]) {
                return false;
            }
        }
        return true;
    }
    return false;
}

#[cfg(test)]
mod valid_expressions {
    use super::*;
    #[test]
    fn literal_is_expression() {
        let mut token_list = vec![Token {
            token_type: TokenType::IntegerLiteral,
            value: Some("1".to_string()),
            name: None,
        }];
        let (node, _) = parse_expression(&mut token_list);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::Expression,
                    children: vec![],
                    parent: None
                }
            ),
            true
        );
    }
}
#[cfg(test)]
mod valid_statements {
    use super::*;
    #[test]
    fn return_statement_is_statement() {
        let mut token_list = vec![
            Token {
                token_type: TokenType::Return,
                value: Some("return".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::IntegerLiteral,
                value: Some("1".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::Semicolon,
                value: Some(";".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_statement(&mut token_list);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::Statement,
                    children: vec![Node {
                        kind: NodeKind::Expression,
                        children: vec![],
                        parent: None
                    }],
                    parent: None
                }
            ),
            true
        );
    }
}
#[cfg(test)]
mod invalid_statements {
    use super::*;
    #[test]
    fn statement_without_semicolon_fails() {
        let mut token_list = vec![
            Token {
                token_type: TokenType::Return,
                value: Some("return".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::IntegerLiteral,
                value: Some("1".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_statement(&mut token_list);
        assert_eq!(node, None);
    }
}
