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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bytes.get(self.cursor) {
            Some(mut curr) => {
                while curr.is_ascii_whitespace() {
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

                    return Some(match bytes.as_slice() {
                        b"fn" => Token::Function,
                        b"let" => Token::Let,
                        b"true" => Token::True,
                        b"false" => Token::False,
                        b"if" => Token::If,
                        b"else" => Token::Else,
                        b"return" => Token::Return,
                        _ => Token::Identifier(bytes),
                    });
                }

                if curr.is_ascii_digit() {
                    let mut bytes = Vec::new();

                    while curr.is_ascii_digit() {
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

                match curr {
                    b'=' | b'!' => {
                        if let Some(next) = self.bytes.get(self.cursor) {
                            if *next == b'=' {
                                self.move_step();

                                return match *curr {
                                    b'=' => Some(Token::Equal),
                                    b'!' => Some(Token::NotEqual),
                                    _ => unimplemented!(),
                                };
                            }
                        }

                        match *curr {
                            b'=' => Some(Token::Assign),
                            b'!' => Some(Token::Bang),
                            _ => unimplemented!(),
                        }
                    }
                    b'*' => Some(Token::Asterisk),
                    b',' => Some(Token::Comma),
                    b'>' => Some(Token::GT),
                    b'{' => Some(Token::LeftBrace),
                    b'(' => Some(Token::LeftParen),
                    b'<' => Some(Token::LT),
                    b'+' => Some(Token::Plus),
                    b'-' => Some(Token::Minus),
                    b'}' => Some(Token::RightBrace),
                    b')' => Some(Token::RightParen),
                    b';' => Some(Token::Semicolon),
                    b'/' => Some(Token::Slash),
                    _ => Some(Token::Illegal(vec![curr.to_owned()])),
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;
    use test::Bencher;

    use pretty_assertions::assert_eq;

    use crate::lexer::Lexer;
    use crate::token::Token;

    extern crate test;

    fn provide_input() -> &'static str {
        r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;
        "#
    }

    #[test]
    fn next() {
        let bytes = provide_input().bytes().collect::<Vec<u8>>();

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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(b"5".to_vec()),
            Token::Semicolon,
            Token::Integer(b"5".to_vec()),
            Token::LT,
            Token::Integer(b"10".to_vec()),
            Token::GT,
            Token::Integer(b"5".to_vec()),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Integer(b"5".to_vec()),
            Token::LT,
            Token::Integer(b"10".to_vec()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Integer(b"10".to_vec()),
            Token::Equal,
            Token::Integer(b"10".to_vec()),
            Token::Semicolon,
            Token::Integer(b"10".to_vec()),
            Token::NotEqual,
            Token::Integer(b"9".to_vec()),
            Token::Semicolon,
        ];

        assert_eq!(
            expected_output,
            Lexer::new(bytes.as_slice()).collect::<Vec<Token>>()
        );
    }

    #[bench]
    fn bench_next(b: &mut Bencher) {
        let bytes = provide_input().bytes().collect::<Vec<u8>>();
        let bytes_as_slice = bytes.as_slice();

        b.iter(|| {
            black_box(Lexer::new(bytes_as_slice).collect::<Vec<Token>>());
        });
    }
}
