use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Whitespace,
    And,
    Or,
    Not,
    True,
    False,
    Variable(String)
}

struct Definitions {
    whitespace: Regex,
    and: Regex,
    or: Regex,
    not: Regex,
    truthy: Regex,
    falsy: Regex,
    variable: Regex
}

pub struct Tokenizer<'a> {
    input: &'a str,
    definitions: Definitions
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let definitions = Definitions {
            whitespace: Regex::new(r"^\s+")
                .expect("invalid whitespace regex"),

            and: Regex::new(r"^(and|[*∧]|\&{1,2})")
                .expect("invalid and regex"),

            or: Regex::new(r"^(or|[+∨]|\|{1,2})")
                .expect("invalid or regex"),

            not: Regex::new(r"^(not|[!¬])")
                .expect("invalid not regex"),

            truthy: Regex::new(r"^(1|true)")
                .expect("invalid truthy regex"),

            falsy: Regex::new(r"^(0|false)")
                .expect("invalid falsy regex"),

            variable: Regex::new(r"^[^*∧+∨!¬\s]+")
                .expect("invalid variable regex")

        };

        return Self { input, definitions }
    }

    // parses all tokens except Token::Variable
    fn get_token(&self, regex: &Regex, token: Token) -> Option<(usize, Token)> {
        return regex.captures(self.input)
            .and_then(|captures| Some((captures[0].len(), token)));
    }

    // parse variabel token
    fn get_var(&self) -> Option<(usize, Token)> {
        return self.definitions.variable.captures(self.input)
            .and_then(|captures| {
                let length = captures[0].len();
                let value = Token::Variable(String::from(&self.input[0..length]));

                return Some((length, value))
            });
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return self.get_token(&self.definitions.whitespace, Token::Whitespace)
            .or(self.get_token(&self.definitions.and, Token::And))
            .or(self.get_token(&self.definitions.or, Token::Or))
            .or(self.get_token(&self.definitions.not, Token::Not))
            .or(self.get_token(&self.definitions.truthy, Token::True))
            .or(self.get_token(&self.definitions.falsy, Token::False))
            .or(self.get_token(&self.definitions.falsy, Token::False))
            .or(self.get_var())
            .and_then(|(length, token)| {
                self.input = &self.input[length..];
                return Some(token);
            })
    }
}
