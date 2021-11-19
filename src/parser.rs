mod parser_matcher;
pub mod parser_nodes;
use crate::lexer::lexer_types::{Token};
pub fn parse(token_list: &[Token]) -> Option<parser_nodes::Node> { 
    return parser_matcher::parse_program(token_list);
}