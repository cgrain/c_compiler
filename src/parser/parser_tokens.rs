#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeKind { 
    Program,
    Function, 
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Node {
    kind: NodeKind,
    parent: Option<Node>,
    children: Vec<Node>,

}
