use crate::token::Token;

pub struct Lexer<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn move_step(&mut self) {
        self.cursor += 1;
    }

    fn is_alphabetic(byte: &u8) -> bool {
        byte.is_ascii_alphabetic() || byte == &b'_'
    }

    fn is_whitespace(byte: &u8) -> bool {
        byte.is_ascii_whitespace()
    }

    fn is_digit(byte: &u8) -> bool {
        byte.is_ascii_digit()
    }

    fn handle_symbol(byte: u8) -> Token {
        match byte {
            b'=' => Token::Assign,
            b',' => Token::Comma,
            b'{' => Token::LeftBrace,
            b'(' => Token::LeftParen,
            b'+' => Token::Plus,
            b'}' => Token::RightBrace,
            b')' => Token::RightParen,
            b';' => Token::Semicolon,
            _ => Token::Illegal(vec![byte]),
        }
    }

    fn handle_ident(bytes: Vec<u8>) -> Option<Token> {
        return match bytes.as_slice() {
            [b'f', b'n'] => Some(Token::Function(bytes)),
            [b'l', b'e', b't'] => Some(Token::Let),
            _ => Some(Token::Identifier(bytes)),
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bytes.get(self.cursor) {
            Some(mut curr) => {
                while Self::is_whitespace(curr) {
                    self.move_step();

                    if let Some(next) = self.bytes.get(self.cursor) {
                        curr = next;
                    } else {
                        return None;
                    }
                }

                if Self::is_alphabetic(curr) {
                    let mut bytes = Vec::new();

                    while Self::is_alphabetic(curr) {
                        self.move_step();

                        bytes.push(curr.to_owned());

                        if let Some(next) = self.bytes.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Self::handle_ident(bytes);
                }

                if Self::is_digit(curr) {
                    let mut bytes = Vec::new();

                    while Self::is_digit(curr) {
                        self.move_step();

                        bytes.push(curr.to_owned());

                        if let Some(next) = self.bytes.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Some(Token::Integer(bytes));
                }

                self.move_step();

                Some(Self::handle_symbol(curr.to_owned()))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let bytes = input.bytes().collect::<Vec<u8>>();

        let expected_output = vec![
            Token::Let,
            Token::Identifier(vec![b'f', b'i', b'v', b'e']),
            Token::Assign,
            Token::Integer(vec![b'5']),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(vec![b't', b'e', b'n']),
            Token::Assign,
            Token::Integer(vec![b'1', b'0']),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(vec![b'a', b'd', b'd']),
            Token::Assign,
            Token::Function(vec![b'f', b'n']),
            Token::LeftParen,
            Token::Identifier(vec![b'x']),
            Token::Comma,
            Token::Identifier(vec![b'y']),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier(vec![b'x']),
            Token::Plus,
            Token::Identifier(vec![b'y']),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(vec![b'r', b'e', b's', b'u', b'l', b't']),
            Token::Assign,
            Token::Identifier(vec![b'a', b'd', b'd']),
            Token::LeftParen,
            Token::Identifier(vec![b'f', b'i', b'v', b'e']),
            Token::Comma,
            Token::Identifier(vec![b't', b'e', b'n']),
            Token::RightParen,
            Token::Semicolon,
        ];

        assert_eq!(
            expected_output,
            Lexer::new(bytes.as_slice()).collect::<Vec<Token>>()
        );
    }

    #[bench]
    fn bench_next(b: &mut Bencher) {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let bytes = input.bytes().collect::<Vec<u8>>();
        let bytes_as_slice = bytes.as_slice();

        b.iter(|| {
            let _ = Lexer::new(bytes_as_slice).collect::<Vec<Token>>();
        });
    }
}
