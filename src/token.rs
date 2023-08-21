#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    ASSIGN(&'a str),
    COMMA(&'a str),
    EOF(&'a str),
    FUNCTION(&'a str),
    IDENT(&'a str),
    ILLEGAL(&'a str),
    INT(&'a str),
    LBRACE(&'a str),
    LET(&'a str),
    LPAREN(&'a str),
    PLUS(&'a str),
    RBRACE(&'a str),
    RPAREN(&'a str),
    SEMICOLON(&'a str),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "=" => Self::ASSIGN(value),
            "+" => Self::PLUS(value),
            "(" => Self::LPAREN(value),
            ")" => Self::RPAREN(value),
            "{" => Self::LBRACE(value),
            "}" => Self::RBRACE(value),
            ";" => Self::SEMICOLON(value),
            "" => Self::EOF(value),
            _ => Self::ILLEGAL(value),
        }
    }
}
