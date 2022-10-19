mod tokenizer;
mod parser;

fn eval(expression: parser::Node) -> bool {
    return match expression {
        parser::Node::Boolean(value) => value,
        parser::Node::And(left, right) => eval(*left) && eval(*right),
        parser::Node::Or(left, right) => eval(*left) || eval(*right),
        parser::Node::Not(node) => !eval(*node)
    };
}

fn main() {
    let expression = std::env::args()
        .skip(1)
        .next()
        .expect("no expression provided");

    let mut prsr = parser::Parser::new(&expression);
    let parsed = prsr.parse()
        .expect("invalid expression");

    let result = eval(parsed);

    println!("{result:?}");
}

#[cfg(test)]
mod test {
    use crate::{parser::Parser, eval};

    #[test]
    fn creating_parser() {
        let input = "true";
        let _parser = Parser::new(input);
    }

    fn parse_and_eval(input: &str) -> Option<bool> {
        let mut parser = Parser::new(input);
        return parser.parse()
            .and_then(|node| Some(eval(node)));
    }

    #[test]
    fn test_bool() {
        assert_eq!(parse_and_eval("1"), Some(true));
        assert_eq!(parse_and_eval("true"), Some(true));

        assert_eq!(parse_and_eval("0"), Some(false));
        assert_eq!(parse_and_eval("false"), Some(false));
    }

    #[test]
    fn test_and() {
        assert_eq!(parse_and_eval("0 * 0"), Some(false));
        assert_eq!(parse_and_eval("0 * 1"), Some(false));
        assert_eq!(parse_and_eval("1 * 0"), Some(false));
        assert_eq!(parse_and_eval("1 * 1"), Some(true));

        assert_eq!(parse_and_eval("0 ∧ 0"), Some(false));
        assert_eq!(parse_and_eval("0 ∧ 1"), Some(false));
        assert_eq!(parse_and_eval("1 ∧ 0"), Some(false));
        assert_eq!(parse_and_eval("1 ∧ 1"), Some(true));

        assert_eq!(parse_and_eval("0 & 0"), Some(false));
        assert_eq!(parse_and_eval("0 & 1"), Some(false));
        assert_eq!(parse_and_eval("1 & 0"), Some(false));
        assert_eq!(parse_and_eval("1 & 1"), Some(true));

        assert_eq!(parse_and_eval("0 && 0"), Some(false));
        assert_eq!(parse_and_eval("0 && 1"), Some(false));
        assert_eq!(parse_and_eval("1 && 0"), Some(false));
        assert_eq!(parse_and_eval("1 && 1"), Some(true));
    }

    #[test]
    fn test_or() {
        assert_eq!(parse_and_eval("0 + 0"), Some(false));
        assert_eq!(parse_and_eval("0 + 1"), Some(true));
        assert_eq!(parse_and_eval("1 + 0"), Some(true));
        assert_eq!(parse_and_eval("1 + 1"), Some(true));

        assert_eq!(parse_and_eval("0 ∨ 0"), Some(false));
        assert_eq!(parse_and_eval("0 ∨ 1"), Some(true));
        assert_eq!(parse_and_eval("1 ∨ 0"), Some(true));
        assert_eq!(parse_and_eval("1 ∨ 1"), Some(true));

        assert_eq!(parse_and_eval("0 | 0"), Some(false));
        assert_eq!(parse_and_eval("0 | 1"), Some(true));
        assert_eq!(parse_and_eval("1 | 0"), Some(true));
        assert_eq!(parse_and_eval("1 | 1"), Some(true));

        assert_eq!(parse_and_eval("0 || 0"), Some(false));
        assert_eq!(parse_and_eval("0 || 1"), Some(true));
        assert_eq!(parse_and_eval("1 || 0"), Some(true));
        assert_eq!(parse_and_eval("1 || 1"), Some(true));
    }

    #[test]
    fn test_not() {
        assert_eq!(parse_and_eval("!true"), Some(false));
        assert_eq!(parse_and_eval("!false"), Some(true));

        assert_eq!(parse_and_eval("¬true"), Some(false));
        assert_eq!(parse_and_eval("¬false"), Some(true));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(parse_and_eval("1 + 0 * 1"), Some(true));
        assert_eq!(parse_and_eval("0 * 1 + 1"), Some(true));

        assert_eq!(parse_and_eval("!1 + 0 * 1"), Some(false));
        assert_eq!(parse_and_eval("0 * 1 + !1"), Some(false));
    }

    #[test]
    fn test_parenthese() {
        assert_eq!(parse_and_eval("(1)"), Some(true));
        assert_eq!(parse_and_eval("(0)"), Some(false));

        assert_eq!(parse_and_eval("0 * (1 + 1)"), Some(false));
        assert_eq!(parse_and_eval("(1 + 1) * 0"), Some(false));
    }
}
