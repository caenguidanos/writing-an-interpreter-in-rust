use crate::token::Token;

pub struct Lexer<'a> {
    input: std::vec::IntoIter<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.split(' ').collect::<Vec<&'a str>>().into_iter(),
        }
    }

    pub fn build(&mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];

        for token in self {
            tokens.push(token);
        }

        tokens
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(s) = self.input.next() else {
            return None;
        };

        Some(Token::from(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
        "#;

        let output = vec![
            Token::LET("let"),
            Token::IDENT("five"),
            Token::ASSIGN("="),
            Token::INT("5"),
            Token::SEMICOLON(";"),
            Token::LET("let"),
            Token::IDENT("ten"),
            Token::ASSIGN("="),
            Token::INT("10"),
            Token::SEMICOLON(";"),
            Token::LET("let"),
            Token::IDENT("add"),
            Token::ASSIGN("="),
            Token::FUNCTION("fn"),
            Token::LPAREN("("),
            Token::IDENT("x"),
            Token::COMMA(","),
            Token::IDENT("y"),
            Token::RPAREN(")"),
            Token::LBRACE("{"),
            Token::IDENT("x"),
            Token::PLUS("+"),
            Token::IDENT("y"),
            Token::SEMICOLON(";"),
            Token::RBRACE("}"),
            Token::SEMICOLON(";"),
            Token::LET("let"),
            Token::IDENT("result"),
            Token::ASSIGN("="),
            Token::IDENT("add"),
            Token::LPAREN("("),
            Token::IDENT("five"),
            Token::COMMA(","),
            Token::IDENT("ten"),
            Token::RPAREN(")"),
            Token::SEMICOLON(";"),
            Token::EOF(""),
        ];

        let mut lexer = Lexer::new(input.trim());

        assert_eq!(lexer.build(), output);
    }
}
