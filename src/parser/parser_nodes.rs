#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind { 
    Program,
    Function, 
    FunctionHeader,
    Block,
    StatementList,
    Statement,
    Expression,
    Type,
    Identifier,
    Literal,
    Operator,
    Keyword,
    Punctuation,
    Whitespace,
    Comment,
    Unknown,

}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub  parent: Option<Box<Node>>,
    pub children: Vec<Node>,

}
