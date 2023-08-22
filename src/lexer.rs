use crate::token::Token;

pub struct Lexer<'a> {
    chars: &'a Vec<char>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a Vec<char>) -> Self {
        Self { chars, cursor: 0 }
    }

    fn step(&mut self) {
        self.cursor += 1;
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

    fn handle_symbol(ch: char) -> Token {
        match ch {
            '=' => Token::Assign(vec![ch]),
            ',' => Token::Comma(vec![ch]),
            '{' => Token::LeftBrace(vec![ch]),
            '(' => Token::LeftParen(vec![ch]),
            '+' => Token::Plus(vec![ch]),
            '}' => Token::RightBrace(vec![ch]),
            ')' => Token::RightParen(vec![ch]),
            ';' => Token::Semicolon(vec![ch]),
            _ => Token::Illegal(vec![ch]),
        }
    }

    fn handle_ident(chars: Vec<char>) -> Option<Token> {
        return match chars.as_slice() {
            ['f', 'n'] => Some(Token::Function(chars)),
            ['l', 'e', 't'] => Some(Token::Let(chars)),
            _ => Some(Token::Identifier(chars)),
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.get(self.cursor) {
            Some(mut curr) => {
                while Self::is_whitespace(curr) {
                    self.step();

                    if let Some(next) = self.chars.get(self.cursor) {
                        curr = next;
                    } else {
                        return None;
                    }
                }

                if Self::is_alphabetic(curr) {
                    let mut chars = Vec::new();

                    while Self::is_alphabetic(curr) {
                        self.step();

                        chars.push(curr.to_owned());

                        if let Some(next) = self.chars.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Self::handle_ident(chars);
                }

                if Self::is_digit(curr) {
                    let mut chars = Vec::new();

                    while Self::is_digit(curr) {
                        self.step();

                        chars.push(curr.to_owned());

                        if let Some(next) = self.chars.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Some(Token::Integer(chars));
                }

                self.step();

                Some(Self::handle_symbol(curr.to_owned()))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

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

        let input_as_chars = input.chars().collect::<Vec<char>>();

        let expected_output = vec![
            Token::Let(vec!['l', 'e', 't']),
            Token::Identifier(vec!['f', 'i', 'v', 'e']),
            Token::Assign(vec!['=']),
            Token::Integer(vec!['5']),
            Token::Semicolon(vec![';']),
            Token::Let(vec!['l', 'e', 't']),
            Token::Identifier(vec!['t', 'e', 'n']),
            Token::Assign(vec!['=']),
            Token::Integer(vec!['1', '0']),
            Token::Semicolon(vec![';']),
            Token::Let(vec!['l', 'e', 't']),
            Token::Identifier(vec!['a', 'd', 'd']),
            Token::Assign(vec!['=']),
            Token::Function(vec!['f', 'n']),
            Token::LeftParen(vec!['(']),
            Token::Identifier(vec!['x']),
            Token::Comma(vec![',']),
            Token::Identifier(vec!['y']),
            Token::RightParen(vec![')']),
            Token::LeftBrace(vec!['{']),
            Token::Identifier(vec!['x']),
            Token::Plus(vec!['+']),
            Token::Identifier(vec!['y']),
            Token::Semicolon(vec![';']),
            Token::RightBrace(vec!['}']),
            Token::Semicolon(vec![';']),
            Token::Let(vec!['l', 'e', 't']),
            Token::Identifier(vec!['r', 'e', 's', 'u', 'l', 't']),
            Token::Assign(vec!['=']),
            Token::Identifier(vec!['a', 'd', 'd']),
            Token::LeftParen(vec!['(']),
            Token::Identifier(vec!['f', 'i', 'v', 'e']),
            Token::Comma(vec![',']),
            Token::Identifier(vec!['t', 'e', 'n']),
            Token::RightParen(vec![')']),
            Token::Semicolon(vec![';']),
        ];

        assert_eq!(
            expected_output,
            Lexer::new(&input_as_chars).collect::<Vec<Token>>()
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

        let input_as_chars = input.chars().collect::<Vec<char>>();

        b.iter(|| {
            black_box(Lexer::new(&input_as_chars).collect::<Vec<Token>>());
        });
    }
}
