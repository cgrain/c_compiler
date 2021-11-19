use crate::parser::parser_nodes::{Node, NodeKind};

pub fn emit(program_node: &Node) -> String {
    return emit_program(program_node);
}
pub fn emit_program(program_node: &Node) -> String {
    // println!("{}", program_node.to_string());
    return "".to_string();
}
fn emit_expression(expression_node: &Node) -> String {
    return emit_integer_literal(expression_node);
}

fn emit_integer_literal(integer_literal_node: &Node) -> String {
    match &integer_literal_node {
        Node {
            kind: NodeKind::Expression,
            value: Some(value),
            ..
        } => {
            return format!("    mov ${}, %rax \n", value);
        }
        _ => {
            panic!("integer_literal_node.value is None");
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
    return "".to_string();
}
#[allow(unused)]
fn emit_declaration(declaration_node: &Node) -> String {
    return "".to_string();
}
fn emit_type(type_node: &Node) -> String {
    return "".to_string();
}
fn emit_block(block_node: &Node) -> String {
    let mut result = "".to_string();
    for statement_node in block_node.children.iter() {
        let inner_res = emit_statement(statement_node);
        result = format!("{}{}", result, inner_res);
    }
    return result;
}
fn emit_function(function_node: &Node) -> String {

    return "".to_string();
}
fn emit_functionheader(functionheader_node: &Node) -> String {
    match functionheader_node {
        Node { 
            kind: NodeKind::FunctionHeader,
            ..
        }  => {
            
            return "".to_string();
        }
        _ => {
            panic!("functionheader_node.kind is not FunctionHeader");
        }
    }
}

pub fn emit_test(prgoram_node: Node) {
    let answer_to_all_the_questions_asm = " 
    .globl main
    main:
    mov rax, 42
    ret
    ";
    println!("{}", answer_to_all_the_questions_asm);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser_nodes::{Node, NodeKind};
    use crate::lexer::lexer_types::{Token};
    #[test]
    fn test_emit_program() {
        let program_node = Node {
            kind: NodeKind::Program,
            children: vec![],
            value: None,
            parent: None,
        };
        let result = emit_program(&program_node);
        assert_eq!(result, "".to_string());
    }
    #[test]
    fn test_emit_expression() {
        let expression_node = Node {
            kind: NodeKind::Expression,
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
            kind: NodeKind::Expression,
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
                    kind: NodeKind::Expression,
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

}
