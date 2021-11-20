use super::parser_match_expression;

use super::parser_nodes::*;
use crate::lexer::lexer_types::*;

fn match_token(token_list: &[Token], match_token_type: TokenType) -> bool {
    match token_list.get(0) {
        Some(Token { token_type: tt, .. }) => tt == &match_token_type,
        _ => return false,
    }
}

fn parse_expression(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    return parser_match_expression::parse_expression(token_list);
}
fn parse_statement(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    return parse_return_statement(token_list);
}

fn parse_return_statement(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <ReturnStatement> := <return> <Expression> <SemiColon>
    if match_token(token_list, TokenType::Return) {
        let (expression, token_list_ret) = parse_expression(&token_list[1..]);
        if expression == None {
            println!("Error: Expected expression after return");
            return (None, token_list);
        }
        if match_token(token_list_ret, TokenType::Semicolon) {
            return (
                Some(Node {
                    kind: NodeKind::Statement,
                    parent: None,
                    children: vec![expression.unwrap()],
                    value: None,
                }),
                &token_list_ret[1..],
            );
        }
    }
    return (None, token_list);
}

fn parse_statement_list(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <StatementList> := <Statement> <StatementList>
    let mut statement_list = vec![];
    let mut token_list_cont = token_list;
    loop {
        let (statement, token_return) = parse_statement(token_list_cont);
        if statement == None {
            break;
        }
        token_list_cont = token_return;
        statement_list.push(statement.unwrap());
    }
    return (
        Some(Node {
            kind: NodeKind::StatementList,
            parent: None,
            children: statement_list,
            value: None,
        }),
        token_list_cont,
    );
}

fn parse_block(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Block> := <LeftBrace> <StatementList> <RightBrace>
    if !match_token(token_list, TokenType::OpenBrace) {
        return (None, token_list);
    }
    println!("{:?}", token_list);
    println!("Parsing block");
    let (statement_list, token_list_ret) = parse_statement_list(&token_list[1..]);
    if statement_list == None {
        println!("Error: Expected statement list after block");
        return (None, token_list);
    }
    println!("{:?}", token_list_ret);
    if !match_token(token_list_ret, TokenType::CloseBrace) {
        return (None, token_list);
    }
    return (
        Some(Node {
            kind: NodeKind::Block,
            parent: None,
            children: vec![statement_list.unwrap()],
            value: None,
        }),
        token_list,
    );
}

fn parse_parameter_list(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // TODO
    return (None, &token_list);
}

fn parse_function_header(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <FunctionHeader> := <Identifier> <Identifier> <OpenParen> <ParameterList> <CloseParen> <Block>
    if !match_token(token_list, TokenType::Int) {
        return (None, token_list);
    }
    if !match_token(&token_list[1..], TokenType::Identifier) {
        return (None, token_list);
    }
    if !match_token(&token_list[2..], TokenType::OpenParenthesis) {
        return (None, token_list);
    }
    let (identifier, _) = parse_identifier(&token_list[1..]);
    let ident_unwrap = identifier.unwrap();
    let (_parameter_list, token_list_ret) = parse_parameter_list(&token_list[3..]);
    if !match_token(token_list_ret, TokenType::CloseParenthesis) {
        return (None, token_list);
    }
    return (
        Some(Node {
            kind: NodeKind::FunctionHeader,
            parent: None,
            children: vec![ident_unwrap],
            value: None,
        }),
        &token_list_ret[1..],
    );
}
fn parse_function(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Function> := <FunctionHeader> <Block>
    let (function_header, token_list_ret) = parse_function_header(token_list);
    if function_header == None {
        println!("Error: Expected function header");
        return (None, token_list);
    }
    let (block, token_list_ret2) = parse_block(token_list_ret);
    if block == None {
        return (None, token_list);
    }
    return (
        Some(Node {
            kind: NodeKind::Function,
            parent: None,
            value: None,
            children: vec![function_header.unwrap(), block.unwrap()],
        }),
        token_list_ret2,
    );
}

fn parse_identifier(token_list: &[Token]) -> (Option<Node>, &[Token]) {
    // <Identifier> := <Identifier>
    if !match_token(token_list, TokenType::Identifier) {
        println!("Error: Expected identifier");
        return (None, token_list);
    }
    return (
        Some(Node {
            kind: NodeKind::Identifier,
            parent: None,
            children: vec![],
            value: token_list.get(0).unwrap().name.clone(),
        }),
        &token_list[1..],
    );
}

#[allow(unused)]
fn parse_class(_token_list: &[Token]) -> (Option<Node>, &[Token]) {
    todo!();
}
pub fn parse_program(token_list: &[Token]) -> Option<Node> {
    let (node, _tokens) = parse_function(token_list);
    return node;
}
#[allow(unused)]
fn equal_nodes(node1: &Node, node2: &Node) -> bool {
    if node1.kind == node2.kind
        && node1.children.len() == node2.children.len()
        && node1.value == node2.value
    {
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
                    value: Some("1".to_string()),
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
                    value: None,
                    children: vec![Node {
                        kind: NodeKind::Expression,
                        value: Some("1".to_string()),
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

#[cfg(test)]
mod valid_function_header {
    use super::*;
    #[test]
    fn valid_function_header() {
        let tokenlist = vec![
            Token {
                token_type: TokenType::Int,
                value: Some("int".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::Identifier,
                value: Some("main".to_string()),
                name: Some("main".to_string()),
            },
            Token {
                token_type: TokenType::OpenParenthesis,
                value: Some("(".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::CloseParenthesis,
                value: Some(")".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_function_header(&tokenlist);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::FunctionHeader,
                    value: None,
                    children: vec![Node {
                        kind: NodeKind::Identifier,
                        value: Some("main".to_string()),
                        children: vec![],
                        parent: None
                    },],
                    parent: None
                }
            ),
            true
        );
    }
}
#[cfg(test)]
mod valid_function {
    use super::*;
    #[test]
    fn valid_block() {
        let tokenlist = vec![
            Token {
                token_type: TokenType::OpenBrace,
                value: Some("{".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::CloseBrace,
                value: Some("}".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_block(&tokenlist);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::Block,
                    value: None,
                    children: vec![Node {
                        kind: NodeKind::StatementList,
                        value: None,
                        children: vec![],
                        parent: None
                    }],
                    parent: None
                }
            ),
            true
        );
    }
    #[test]
    fn valid_function() {
        let tokenlist = vec![
            Token {
                token_type: TokenType::Int,
                value: Some("int".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::Identifier,
                value: Some("main".to_string()),
                name: Some("main".to_string()),
            },
            Token {
                token_type: TokenType::OpenParenthesis,
                value: Some("(".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::CloseParenthesis,
                value: Some(")".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::OpenBrace,
                value: Some("{".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::CloseBrace,
                value: Some("}".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_function(&tokenlist);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::Function,
                    value: None,
                    children: vec![
                        Node {
                            kind: NodeKind::FunctionHeader,
                            value: None,
                            children: vec![Node {
                                kind: NodeKind::Identifier,
                                value: Some("main".to_string()),
                                children: vec![],
                                parent: None
                            },],
                            parent: None
                        },
                        Node {
                            kind: NodeKind::Block,
                            value: None,
                            children: vec![Node {
                                kind: NodeKind::StatementList,
                                value: None,
                                children: vec![],
                                parent: None
                            }],
                            parent: None
                        }
                    ],
                    parent: None
                }
            ),
            true
        );
    }
    #[test]
    fn valid_function_with_return() {
        // int main() { return 1; }
        let tokenlist = vec![
            Token {
                token_type: TokenType::Int,
                value: Some("int".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::Identifier,
                value: Some("main".to_string()),
                name: Some("main".to_string()),
            },
            Token {
                token_type: TokenType::OpenParenthesis,
                value: Some("(".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::CloseParenthesis,
                value: Some(")".to_string()),
                name: None,
            },
            Token {
                token_type: TokenType::OpenBrace,
                value: Some("{".to_string()),
                name: None,
            },
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
            Token {
                token_type: TokenType::CloseBrace,
                value: Some("}".to_string()),
                name: None,
            },
        ];
        let (node, _) = parse_function(&tokenlist);
        assert_eq!(
            equal_nodes(
                &node.unwrap(),
                &Node {
                    kind: NodeKind::Function,
                    value: None,
                    children: vec![
                        Node {
                            kind: NodeKind::FunctionHeader,
                            children: vec![Node {
                                kind: NodeKind::Identifier,
                                value: Some("main".to_string()),
                                children: vec![],
                                parent: None
                            },],
                            parent: None,
                            value: None,
                        },
                        Node {
                            kind: NodeKind::Block,
                            value: None,
                            children: vec![Node {
                                kind: NodeKind::StatementList,
                                value: None,
                                children: vec![Node {
                                    kind: NodeKind::Statement,
                                    value: None,
                                    children: vec![Node {
                                        kind: NodeKind::Expression,
                                        value: Some("1".to_string()),
                                        children: vec![],
                                        parent: None
                                    }],
                                    parent: None
                                }],
                                parent: None
                            }],
                            parent: None
                        }
                    ],
                    parent: None
                }
            ),
            true
        );
    }
}
