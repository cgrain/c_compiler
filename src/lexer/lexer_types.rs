#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    SemiColon,
    Int,
    Return,
    Identifier,
    IntegerLiteral,
    WhiteSpace,
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}