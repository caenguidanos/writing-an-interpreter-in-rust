use crate::token::{Token, TokenMetadata};

pub struct Lexer<'a> {
    chars: &'a Vec<char>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a Vec<char>) -> Self {
        Self { chars, cursor: 0 }
    }

    fn is_alphabetic(ch: &char) -> bool {
        ch.is_ascii_alphabetic() || ch == &'_'
    }

    fn is_whitespace(ch: &char) -> bool {
        ch.is_ascii_whitespace()
    }

    fn is_digit(ch: &char) -> bool {
        ch.is_ascii_digit()
    }

    fn handle_symbol(ch: char) -> Option<Token> {
        match ch {
            '=' => Some(Token::Assign(TokenMetadata {
                literal: String::from(ch),
            })),
            ',' => Some(Token::Comma(TokenMetadata {
                literal: String::from(ch),
            })),
            '{' => Some(Token::LeftBrace(TokenMetadata {
                literal: String::from(ch),
            })),
            '(' => Some(Token::LeftParen(TokenMetadata {
                literal: String::from(ch),
            })),
            '+' => Some(Token::Plus(TokenMetadata {
                literal: String::from(ch),
            })),
            '}' => Some(Token::RightBrace(TokenMetadata {
                literal: String::from(ch),
            })),
            ')' => Some(Token::RightParen(TokenMetadata {
                literal: String::from(ch),
            })),
            ';' => Some(Token::Semicolon(TokenMetadata {
                literal: String::from(ch),
            })),
            _ => None,
        }
    }

    fn handle_ident(literal: String) -> Option<Token> {
        return match literal.as_str() {
            "fn" => Some(Token::Function(TokenMetadata { literal })),
            "let" => Some(Token::Let(TokenMetadata { literal })),
            _ => Some(Token::Identifier(TokenMetadata { literal })),
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.get(self.cursor) {
            Some(mut curr) => {
                while Self::is_whitespace(curr) {
                    self.cursor += 1;

                    if let Some(next) = self.chars.get(self.cursor) {
                        curr = next;
                    } else {
                        return None;
                    }
                }

                if Self::is_alphabetic(curr) {
                    let mut literal = String::new();

                    while Self::is_alphabetic(curr) {
                        self.cursor += 1;

                        literal.push(curr.to_owned());

                        if let Some(next) = self.chars.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Self::handle_ident(literal);
                }

                if Self::is_digit(curr) {
                    let mut literal = String::new();

                    while Self::is_digit(curr) {
                        self.cursor += 1;

                        literal.push(curr.to_owned());

                        if let Some(next) = self.chars.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Some(Token::Integer(TokenMetadata { literal }));
                }

                if let Some(symbol_token) = Self::handle_symbol(curr.to_owned()) {
                    self.cursor += 1;

                    return Some(symbol_token);
                }

                self.cursor += 1;

                Some(Token::Illegal(TokenMetadata {
                    literal: String::from(curr.to_owned()),
                }))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenMetadata};

    #[test]
    fn it_must_iterate() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let input_as_chars = input.chars().collect::<Vec<char>>();

        let expected_output = vec![
            Token::Let(TokenMetadata {
                literal: String::from("let"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("five"),
            }),
            Token::Assign(TokenMetadata {
                literal: String::from("="),
            }),
            Token::Integer(TokenMetadata {
                literal: String::from("5"),
            }),
            Token::Semicolon(TokenMetadata {
                literal: String::from(";"),
            }),
            Token::Let(TokenMetadata {
                literal: String::from("let"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("ten"),
            }),
            Token::Assign(TokenMetadata {
                literal: String::from("="),
            }),
            Token::Integer(TokenMetadata {
                literal: String::from("10"),
            }),
            Token::Semicolon(TokenMetadata {
                literal: String::from(";"),
            }),
            Token::Let(TokenMetadata {
                literal: String::from("let"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("add"),
            }),
            Token::Assign(TokenMetadata {
                literal: String::from("="),
            }),
            Token::Function(TokenMetadata {
                literal: String::from("fn"),
            }),
            Token::LeftParen(TokenMetadata {
                literal: String::from("("),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("x"),
            }),
            Token::Comma(TokenMetadata {
                literal: String::from(","),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("y"),
            }),
            Token::RightParen(TokenMetadata {
                literal: String::from(")"),
            }),
            Token::LeftBrace(TokenMetadata {
                literal: String::from("{"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("x"),
            }),
            Token::Plus(TokenMetadata {
                literal: String::from("+"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("y"),
            }),
            Token::Semicolon(TokenMetadata {
                literal: String::from(";"),
            }),
            Token::RightBrace(TokenMetadata {
                literal: String::from("}"),
            }),
            Token::Semicolon(TokenMetadata {
                literal: String::from(";"),
            }),
            Token::Let(TokenMetadata {
                literal: String::from("let"),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("result"),
            }),
            Token::Assign(TokenMetadata {
                literal: String::from("="),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("add"),
            }),
            Token::LeftParen(TokenMetadata {
                literal: String::from("("),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("five"),
            }),
            Token::Comma(TokenMetadata {
                literal: String::from(","),
            }),
            Token::Identifier(TokenMetadata {
                literal: String::from("ten"),
            }),
            Token::RightParen(TokenMetadata {
                literal: String::from(")"),
            }),
            Token::Semicolon(TokenMetadata {
                literal: String::from(";"),
            }),
        ];

        assert_eq!(
            expected_output,
            Lexer::new(&input_as_chars).collect::<Vec<Token>>()
        );
    }
}
