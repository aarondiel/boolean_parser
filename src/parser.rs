use crate::tokenizer::{ Tokenizer, Token };

#[derive(Debug, Clone)]
pub enum Node {
    Boolean(bool),
    Or(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>)
}

pub struct Parser<'a> {
    tokens: Tokenizer<'a>,
    peeked: Option<Option<Token>>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        return Self {
            tokens: Tokenizer::new(input),
            peeked: None
        };
    }

    pub fn parse(&mut self) -> Option<Node> {
        self.whitespace();

        if self.peek().is_none() {
            return None;
        }

        return self.expression(0);
    }

    fn expression(&mut self, binding_power: u8) -> Option<Node> {
        let mut left = self.atom()
            .expect("unexpected end of file");

        loop {
            self.whitespace();

            let operator = match self.peek() {
                None => break,
                Some(operator) => operator
            };

            let precedence = Self::get_operator_precedence(&operator)
                .expect("unexpected token");

            if precedence.0 < binding_power {
                break;
            }

            self.next();

            self.whitespace();

            let right = self.expression(precedence.1)
                .expect("unexpected end of file");

            left = match operator {
                Token::And => Node::And(Box::new(left), Box::new(right)),
                Token::Or => Node::Or(Box::new(left), Box::new(right)),
                _ => panic!("unexpected token")
            }
        }

        return Some(left);
    }

    fn atom(&mut self) -> Option<Node> {
        let token = self.peek()
            .expect("unexpected end of file");

        let node = match token {
            Token::True => Some(Node::Boolean(true)),
            Token::False => Some(Node::Boolean(false)),
            _ => None
        };

        if node.is_some() {
            self.next();
        }

        return node;
    }

    fn whitespace(&mut self) {
        if self.peek() != Some(Token::Whitespace) {
            return;
        }

        self.next();
    }

    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.peeked.take() {
            return token;
        }

        return self.tokens.next();
    }

    fn peek(&mut self) -> Option<Token> {
        return self.peeked
            .get_or_insert_with(|| self.tokens.next())
            .clone();
    }

    fn get_operator_precedence(operator: &Token) -> Option<(u8, u8)> {
        return match operator {
            Token::Or => Some((1, 2)),
            Token::And => Some((3, 4)),
            _ => None
        };
    }
}
