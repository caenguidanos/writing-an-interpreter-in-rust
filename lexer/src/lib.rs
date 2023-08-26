use token::Token;

#[derive(Debug)]
pub struct Lexer<'source> {
    input: &'source [u8],
    position: usize,
    read_position: usize,
    ch: Option<&'source u8>,
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source [u8]) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.get(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> Option<&[u8]> {
        let position = self.position;

        while Self::is_digit(self.ch) {
            self.read_char();
        }

        self.input.get(position..self.position)
    }

    fn read_identifier(&mut self) -> Option<&[u8]> {
        let position = self.position;

        while Self::is_letter(self.ch) {
            self.read_char();
        }

        self.input.get(position..self.position)
    }

    fn peek_char(&self) -> Option<&u8> {
        if self.read_position >= self.input.len() {
            return None;
        }

        self.input.get(self.read_position)
    }

    fn is_digit(ch: Option<&u8>) -> bool {
        ch.map(|b| b.is_ascii_digit()).unwrap_or_default()
    }

    fn is_letter(ch: Option<&u8>) -> bool {
        ch.map(|c| c.is_ascii_alphabetic() || *c == b'_')
            .unwrap_or_default()
    }

    fn is_whitespace(ch: Option<&u8>) -> bool {
        ch.map(|b| b.is_ascii_whitespace()).unwrap_or_default()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while Self::is_whitespace(self.ch) {
            self.read_char();
        }

        match self.ch {
            Some(ch) => {
                let token = match ch {
                    b'=' => {
                        if let Some(b'=') = self.peek_char() {
                            self.read_char();
                            Some(Token::Equal)
                        } else {
                            Some(Token::Assign)
                        }
                    }
                    b';' => Some(Token::Semicolon),
                    b'(' => Some(Token::LeftParen),
                    b')' => Some(Token::RightParen),
                    b',' => Some(Token::Comma),
                    b'+' => Some(Token::Plus),
                    b'-' => Some(Token::Minus),
                    b'!' => {
                        if let Some(b'=') = self.peek_char() {
                            self.read_char();
                            Some(Token::NotEqual)
                        } else {
                            Some(Token::Bang)
                        }
                    }
                    b'/' => Some(Token::Slash),
                    b'*' => {
                        if let Some(b'*') = self.peek_char() {
                            self.read_char();
                            Some(Token::Pow)
                        } else {
                            Some(Token::Asterisk)
                        }
                    }
                    b'<' => {
                        if let Some(b'=') = self.peek_char() {
                            self.read_char();
                            Some(Token::LessThanOrEqual)
                        } else {
                            Some(Token::LessThan)
                        }
                    }
                    b'>' => {
                        if let Some(b'=') = self.peek_char() {
                            self.read_char();
                            Some(Token::GreaterThanOrEqual)
                        } else {
                            Some(Token::GreaterThan)
                        }
                    }
                    b'{' => Some(Token::LeftBrace),
                    b'}' => Some(Token::RightBrace),
                    _ => {
                        if Self::is_letter(Some(ch)) {
                            return self.read_identifier().map(|ident| match ident {
                                b"fn" => Token::Function,
                                b"let" => Token::Let,
                                b"true" => Token::True,
                                b"false" => Token::False,
                                b"if" => Token::If,
                                b"else" => Token::Else,
                                b"return" => Token::Return,
                                _ => Token::Identifier(ident.to_owned()),
                            });
                        }

                        if Self::is_digit(Some(ch)) {
                            return self
                                .read_number()
                                .map(|digit| Token::Integer(digit.to_owned()));
                        }

                        Some(Token::Illegal(vec![*ch]))
                    }
                };

                self.read_char();
                token
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;

    fn provide_input() -> &'static str {
        r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result_of_add = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;
            10 <= 10;
            10 >= 10;
            10 ** 10;
        "#
    }

    #[test]
    fn tokenize() {
        let expected_output = vec![
            token::Token::Let,
            token::Token::Identifier(b"five".to_vec()),
            token::Token::Assign,
            token::Token::Integer(b"5".to_vec()),
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Identifier(b"ten".to_vec()),
            token::Token::Assign,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Identifier(b"add".to_vec()),
            token::Token::Assign,
            token::Token::Function,
            token::Token::LeftParen,
            token::Token::Identifier(b"x".to_vec()),
            token::Token::Comma,
            token::Token::Identifier(b"y".to_vec()),
            token::Token::RightParen,
            token::Token::LeftBrace,
            token::Token::Identifier(b"x".to_vec()),
            token::Token::Plus,
            token::Token::Identifier(b"y".to_vec()),
            token::Token::Semicolon,
            token::Token::RightBrace,
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Identifier(b"result_of_add".to_vec()),
            token::Token::Assign,
            token::Token::Identifier(b"add".to_vec()),
            token::Token::LeftParen,
            token::Token::Identifier(b"five".to_vec()),
            token::Token::Comma,
            token::Token::Identifier(b"ten".to_vec()),
            token::Token::RightParen,
            token::Token::Semicolon,
            token::Token::Bang,
            token::Token::Minus,
            token::Token::Slash,
            token::Token::Asterisk,
            token::Token::Integer(b"5".to_vec()),
            token::Token::Semicolon,
            token::Token::Integer(b"5".to_vec()),
            token::Token::LessThan,
            token::Token::Integer(b"10".to_vec()),
            token::Token::GreaterThan,
            token::Token::Integer(b"5".to_vec()),
            token::Token::Semicolon,
            token::Token::If,
            token::Token::LeftParen,
            token::Token::Integer(b"5".to_vec()),
            token::Token::LessThan,
            token::Token::Integer(b"10".to_vec()),
            token::Token::RightParen,
            token::Token::LeftBrace,
            token::Token::Return,
            token::Token::True,
            token::Token::Semicolon,
            token::Token::RightBrace,
            token::Token::Else,
            token::Token::LeftBrace,
            token::Token::Return,
            token::Token::False,
            token::Token::Semicolon,
            token::Token::RightBrace,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Equal,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Semicolon,
            token::Token::Integer(b"10".to_vec()),
            token::Token::NotEqual,
            token::Token::Integer(b"9".to_vec()),
            token::Token::Semicolon,
            token::Token::Integer(b"10".to_vec()),
            token::Token::LessThanOrEqual,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Semicolon,
            token::Token::Integer(b"10".to_vec()),
            token::Token::GreaterThanOrEqual,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Semicolon,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Pow,
            token::Token::Integer(b"10".to_vec()),
            token::Token::Semicolon,
        ];

        let lexer = Lexer::new(provide_input().as_bytes());

        pretty_assertions::assert_eq!(expected_output, lexer.collect::<Vec<token::Token>>());
    }
}
