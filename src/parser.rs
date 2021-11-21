mod parser_matcher;
mod parser_match_expression;
mod parser_match_expression2;
pub mod parser_nodes;
use crate::lexer::lexer_types::{Token};
pub fn parse(token_list: &[Token]) -> Option<parser_nodes::Node> { 
    return parser_matcher::parse_program(token_list);
}