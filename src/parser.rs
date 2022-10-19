use crate::tokenizer::{ Tokenizer, Token };

#[derive(Debug, Clone)]
pub enum Node {
    Boolean(bool),
    Or(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Not(Box<Node>)
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
        let token = self.next()
            .expect("unexpected end of file");

        let mut left = match &token {
            Token::True => Node::Boolean(true),
            Token::False => Node::Boolean(false),
            Token::Not => {
                self.whitespace();

                let precedence = Self::get_prefix_precedence(&token)
                    .expect("get prefix precedence failed");

                let right = self.expression(precedence)
                    .expect("unexpected token");

                Node::Not(Box::new(right))
            },

            Token::LeftParanthese => {
                self.whitespace();

                let left = self.expression(0)
                    .expect("unexpected token");
                    
                assert_eq!(self.next(), Some(Token::RightParanthese));

                left
            }

            _ => panic!("unexpected token")
        };

        loop {
            self.whitespace();

            let operator = match self.peek() {
                None => break,
                Some(operator) => operator
            };

            let precedence = match Self::get_operator_precedence(&operator) {
                Some(precedence) => precedence,
                None => break
            };

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

    fn get_prefix_precedence(operator: &Token) -> Option<u8> {
        return match operator {
            Token::Not => Some(5),
            _ => None
        }
    }

    fn get_operator_precedence(operator: &Token) -> Option<(u8, u8)> {
        return match operator {
            Token::Or => Some((1, 2)),
            Token::And => Some((3, 4)),
            _ => None
        };
    }
}
