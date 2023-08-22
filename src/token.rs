#[derive(PartialEq, Debug)]
pub enum Token {
    Assign(Vec<char>),
    Comma(Vec<char>),
    Function(Vec<char>),
    Identifier(Vec<char>),
    Illegal(Vec<char>),
    Integer(Vec<char>),
    LeftBrace(Vec<char>),
    Let(Vec<char>),
    LeftParen(Vec<char>),
    Plus(Vec<char>),
    RightBrace(Vec<char>),
    RightParen(Vec<char>),
    Semicolon(Vec<char>),
}
