mod calc;

fn main() -> Result<(), calc::Error> {
  // let tokens = calc::Calculator::parse("2 * 2 + 48 / 4");
  // println!("{:?}", tokens);
  // let expr = calc::Calculator::expression(tokens.unwrap());
  // println!("{:?}", expr);
  // let value = calc::Calculator::evaluate(expr);
  // println!("{}", value.unwrap());

  loop {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
      Ok(_) => {
        let tokens = calc::Calculator::parse(&input);
        if tokens.is_err() {
          println!("{:?}", tokens.err().unwrap());
          continue;
        }
        let expr = calc::Calculator::expression(tokens.unwrap());
        if let Some(v) = calc::Calculator::evaluate(expr) {
          println!("{}", v);
        }
      },
      Err(error) => println!("error: {}", error),
    }
  }
}
