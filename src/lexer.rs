use crate::token::Token;

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
            '=' => Some(Token::Assign(vec![ch])),
            ',' => Some(Token::Comma(vec![ch])),
            '{' => Some(Token::LeftBrace(vec![ch])),
            '(' => Some(Token::LeftParen(vec![ch])),
            '+' => Some(Token::Plus(vec![ch])),
            '}' => Some(Token::RightBrace(vec![ch])),
            ')' => Some(Token::RightParen(vec![ch])),
            ';' => Some(Token::Semicolon(vec![ch])),
            _ => None,
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
                    self.cursor += 1;

                    if let Some(next) = self.chars.get(self.cursor) {
                        curr = next;
                    } else {
                        return None;
                    }
                }

                if Self::is_alphabetic(curr) {
                    let mut chars = Vec::new();

                    while Self::is_alphabetic(curr) {
                        self.cursor += 1;

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
                        self.cursor += 1;

                        chars.push(curr.to_owned());

                        if let Some(next) = self.chars.get(self.cursor) {
                            curr = next;
                        } else {
                            return None;
                        }
                    }

                    return Some(Token::Integer(chars));
                }

                if let Some(symbol_token) = Self::handle_symbol(curr.to_owned()) {
                    self.cursor += 1;

                    return Some(symbol_token);
                }

                self.cursor += 1;

                Some(Token::Illegal(vec![curr.to_owned()]))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

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
}
