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
        byte.is_ascii_alphabetic() || *byte == b'_'
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
            b"fn" => Some(Token::Function),
            b"let" => Some(Token::Let),
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

    use std::hint::black_box;
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
            Token::Identifier(b"five".to_vec()),
            Token::Assign,
            Token::Integer(b"5".to_vec()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(b"ten".to_vec()),
            Token::Assign,
            Token::Integer(b"10".to_vec()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(b"add".to_vec()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier(b"x".to_vec()),
            Token::Comma,
            Token::Identifier(b"y".to_vec()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier(b"x".to_vec()),
            Token::Plus,
            Token::Identifier(b"y".to_vec()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(b"result".to_vec()),
            Token::Assign,
            Token::Identifier(b"add".to_vec()),
            Token::LeftParen,
            Token::Identifier(b"five".to_vec()),
            Token::Comma,
            Token::Identifier(b"ten".to_vec()),
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
            black_box(Lexer::new(bytes_as_slice).collect::<Vec<Token>>());
        });
    }
}
