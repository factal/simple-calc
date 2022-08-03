mod calc;

fn main() -> Result<(), calc::Error> {
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
