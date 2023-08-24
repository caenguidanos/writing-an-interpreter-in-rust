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

                    while Self::is_alphabetic(curr) || curr.is_ascii_digit() {
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
