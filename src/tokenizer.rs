#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Whitespace,
    And,
    Or,
    Not,
    Variable(String),
    True,
    False
}

pub struct Tokenizer<'a> {
    input: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        return Self { input }
    }

    fn get_char_type(character: char) -> Option<Token> {
        match character {
            ' ' | '\t' => Some(Token::Whitespace),
            '*' => Some(Token::And),
            '+' => Some(Token::Or),
            '!' => Some(Token::Not),
            '1' => Some(Token::True),
            '0' => Some(Token::False),
            _ => None
        }
    }

    fn get_whitespace_token(&mut self) -> Option<Token> {
        let length = self.input
            .chars()
            .map(Self::get_char_type)
            .take_while(|token_type| *token_type == Some(Token::Whitespace))
            .count();

        if length == 0 {
            return None;
        }

        self.input = &self.input[length..];

        return Some(Token::Whitespace);
    }

    fn get_text_token(&mut self) -> Option<Token> {
        let length = self.input
            .chars()
            .map(Self::get_char_type)
            .take_while(|token_type| token_type.is_none())
            .count();

        if length == 0 {
            return None;
        }

        let value = String::from(&self.input[0..length]);
        self.input = &self.input[length..];

        return Some(Token::Variable(value));
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.input.chars().next() {
            None => return None,
            Some(character) => Self::get_char_type(character)
        };

        if let Some(single_token) = &token {
            if *single_token == Token::Whitespace {
                return self.get_whitespace_token();
            }

            self.input = &self.input[1..];
            return token;
        }

        let token = self.get_text_token()
            .expect("get_text_token should not return empty");

        if let Token::Variable(name) = &token {
            match name.as_str() {
                "true" => return Some(Token::True),
                "false" => return Some(Token::False),
                _ => {}
            };
        }

        return Some(token);
    }
}
