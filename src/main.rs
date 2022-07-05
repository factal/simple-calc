mod calc;

fn main() {
  let tokens = calc::Calculator::parse("2 * 2 + 48 / 4");
  println!("{:?}", tokens);
  let expr = calc::Calculator::expression(tokens.unwrap());
  println!("{:?}", expr);
}
