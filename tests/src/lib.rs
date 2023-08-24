#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use lexer::{Lexer, Token};

    fn provide_input() -> &'static str {
        r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result2 = add(five, ten);
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
    fn tokenize() {
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
            Token::Identifier(b"result2".to_vec()),
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
}
