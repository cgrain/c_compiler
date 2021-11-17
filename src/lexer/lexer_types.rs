#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Int,
    Return,
    Identifier,
    IntegerLiteral,
    WhiteSpace,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: Option<String>, 
    pub value: Option<String>,
}