pub struct Parser<'source, 'lexer> {
    lexer: &'lexer mut lexer::Lexer<'source>,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
}

impl<'source, 'lexer> Parser<'source, 'lexer> {
    pub fn new(lexer: &'lexer mut lexer::Lexer<'source>) -> Self {
        let mut parser = Self {
            lexer,
            curr_token: None,
            peek_token: None,
        };

        parser.next();
        parser.next();

        parser
    }

    pub fn parse(&self) -> ast::Program {
        todo!()
    }
}

impl<'source, 'lexer> Iterator for Parser<'source, 'lexer> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
        self.peek_token.clone()
    }
}
