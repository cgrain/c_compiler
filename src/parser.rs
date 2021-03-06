mod parser_matcher;
mod parser_match_expression;
pub mod parser_nodes;
use crate::lexer::lexer_types::{Token};
pub fn parse(token_list: &[Token]) -> Option<parser_nodes::Node> { 
    return parser_matcher::parse_program(token_list);
}