#[allow(unused)]
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
    Plus,
    Minus,
    Star,
    Slash,
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseRightRotate,
    BitwiseLeftRotate,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Not,
    Ampersand,
    Increment,
    Decrement,
    StringLiteral,
    Keyword,
    Percent,

}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: Option<String>, 
    pub value: Option<String>,
}