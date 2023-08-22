#[derive(PartialEq, Debug)]
pub enum Token {
    Assign,
    Comma,
    Function(Vec<u8>),
    Identifier(Vec<u8>),
    Illegal(Vec<u8>),
    Integer(Vec<u8>),
    LeftBrace,
    Let,
    LeftParen,
    Plus,
    RightBrace,
    RightParen,
    Semicolon,
}
