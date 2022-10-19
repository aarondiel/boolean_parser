mod tokenizer;
mod parser;

fn eval(expression: parser::Node) -> bool {
    return match expression {
        parser::Node::Boolean(value) => value,
        parser::Node::And(left, right) => eval(*left) && eval(*right),
        parser::Node::Or(left, right) => eval(*left) || eval(*right)
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
