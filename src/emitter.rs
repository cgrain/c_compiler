use crate::parser::parser_nodes::{Node, NodeKind};

pub fn emit(program_node: &Node) -> String {
    return emit_program(program_node);
}
pub fn emit_program(program_node: &Node) -> String {
    // println!("{}", program_node.to_string());
    return emit_function(program_node)
}
fn emit_expression(expression_node: &Node) -> String {
    return emit_integer_literal(expression_node);
}

fn emit_integer_literal(integer_literal_node: &Node) -> String {
    match &integer_literal_node {
        Node {
            kind: NodeKind::Literal,
            value: Some(value),
            ..
        } => {
            return format!("    mov ${}, %rax \n", value);
        }
        Node { 
            kind: NodeKind::Literal,
            .. 
        } => {
            panic!("integer_literal_node.value is None");
        }
        _ => {

            panic!("unexpected node: {:?} ", integer_literal_node.kind);
            
        }
    }
}
fn emit_statement(statement_node: &Node) -> String {
    // Only Return (for now)
    match &statement_node.kind {
        NodeKind::Statement => {
            let expression = emit_expression(&statement_node.children[0]);
            return format!("{}    ret\n", expression);
        }
        _ => {
            panic!("statement_node.kind is not Return");
        }
    }
}
#[allow(unused)]
fn emit_declaration(declaration_node: &Node) -> String {
    return "".to_string();
}
fn emit_type(type_node: &Node) -> String {
    return "".to_string();
}
fn emit_block(block_node: &Node) -> String {
    match &block_node.kind {
        NodeKind::Block => {
            if block_node.children.len() == 0 {
                return "".to_string();
            }
            return emit_statement_list(block_node.children.get(0).unwrap());
        }
        _ => {
            panic!("block_node.kind is not Block");
        }
    }
}
fn emit_statement_list(block_node: &Node) -> String {
    let mut result = "".to_string();
    for statement_node in block_node.children.iter() {
        let inner_res = emit_statement(statement_node);
        result = format!("{}{}", result, inner_res);
    }
    return result;
}


fn emit_function(function_node: &Node) -> String {
    let mut function_name = "".to_string();
    let mut function_body = "".to_string();
    for child in function_node.children.iter() {
        match &child.kind {
            NodeKind::FunctionHeader => {
                function_name = emit_functionheader(child);
            }
            NodeKind::Block => {
                function_body = emit_block(child);
            }
            _ => {
                panic!("function_node.kind is not FunctionName, FunctionArgs, or FunctionBody");
            }
        }
    }
    let result = format!(
        "{} {}", function_name, function_body
    );
    return result
}
fn emit_functionheader(functionheader_node: &Node) -> String {
    match functionheader_node {
        Node { 
            kind: NodeKind::FunctionHeader,
            ..
        }  => {
            let ident = emit_function_ident(&functionheader_node.children[0], true);
            return format!("{}", ident);
        }
        _ => {
            panic!("functionheader_node.kind is not FunctionHeader");
        }
    }
}
pub fn emit_function_ident(identifier_node: &Node, globl: bool ) -> String {
    let ident_name = &identifier_node.value.as_ref().unwrap().clone();
    if globl {
        return format!("    .globl {}\n {}:\n", ident_name, ident_name);
    } else { 
        panic!("emit_function_ident: globl is false");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser_nodes::{Node, NodeKind};
    #[test]
    fn test_emit_expression() {
        let expression_node = Node {
            kind: NodeKind::Literal,
            children: vec![],
            value: Some("42".to_string()),
            parent: None,
        };
        let result = emit_expression(&expression_node);
        assert_eq!(result, "    mov $42, %rax \n".to_string());
    }
    #[test]
    fn test_emit_integer_literal() {
        let integer_literal_node = Node {
            kind: NodeKind::Literal,
            children: vec![],
            value: Some("42".to_string()),
            parent: None,
        };
        let result = emit_integer_literal(&integer_literal_node);
        assert_eq!(result, "    mov $42, %rax \n".to_string());
    }
    #[test]
    fn test_emit_statement() {
        let statement_node = Node {
            kind: NodeKind::Statement,
            children: vec![
                Node {
                    kind: NodeKind::Literal,
                    children: vec![],
                    value: Some("42".to_string()),
                    parent: None,
                },
            ],
            value: None,
            parent: None,
        };
        let result = emit_statement(&statement_node);
        assert_eq!(result, "    mov $42, %rax \n    ret\n".to_string());
    }
    #[test]
    fn test_emit_function_header() { 
        let functionheader_node = Node {
            kind: NodeKind::FunctionHeader,
            children: vec![
                Node {
                    kind: NodeKind::Identifier,
                    children: vec![],
                    value: Some("main".to_string()),
                    parent: None,
                },
            ],
            value: None,
            parent: None,
        };
        let result = emit_functionheader(&functionheader_node);
        assert_eq!(result, "    .globl main\n main:\n".to_string());
    }
    #[test]
    fn test_emit_function() {
        let function_node = Node {
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
                                kind: NodeKind::Literal,
                                value: Some("42".to_string()),
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
        };
        let result = emit_function(&function_node);
        println!("{}",&result);
        assert_eq!(result, "    .globl main\n main:\n     mov $42, %rax \n    ret\n".to_string());
    }

}
