use super::parser_nodes::*;
use crate::lexer::lexer_types::*;

pub fn parse_expression(token_list: &Vec<Token>) -> (Option<Node>, &Vec<Token>) {
    // <Expression> := int

    if let Some(token) = token_list.get(0) {
        match token.token_type {
            TokenType::IntegerLiteral => {
                return (
                    Some(Node {
                        kind: NodeKind::Expression,
                        parent: None,
                        children: vec![],
                    }),
                    token_list,
                );
            }
            _ => {
                return (None, token_list);
            }
        }
    } else {
        return (None, token_list);
    }
}
pub fn parse_statement(token_list: &Vec<Token>) -> (Option<Node>, Vec<Token>) {
    todo!()
}
pub fn parse_block(token_list: &Vec<Token>) -> (Option<Node>, Vec<Token>) {
    todo!()
}
pub fn parse_function(token_list: &Vec<Token>) -> (Option<Node>, Vec<Token>) {
    todo!()
}
pub fn parse_class(token_list: &Vec<Token>) -> (Option<Node>, Vec<Token>) {
    todo!()
}
pub fn parse_program(token_list: &Vec<Token>) -> (Option<Node>, Vec<Token>) {
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
        let token_list = vec![Token {
            token_type: TokenType::IntegerLiteral,
            value: Some("1".to_string()),
            name: None,
        }];
        let (node, _) = parse_expression(&token_list);
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
