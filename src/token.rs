#[derive(PartialEq, Debug)]
pub struct TokenMetadata {
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Assign(TokenMetadata),
    Comma(TokenMetadata),
    Function(TokenMetadata),
    Identifier(TokenMetadata),
    Illegal(TokenMetadata),
    Integer(TokenMetadata),
    LeftBrace(TokenMetadata),
    Let(TokenMetadata),
    LeftParen(TokenMetadata),
    Plus(TokenMetadata),
    RightBrace(TokenMetadata),
    RightParen(TokenMetadata),
    Semicolon(TokenMetadata),
}
