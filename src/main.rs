extern crate clap;
use clap::{Arg, App};

// unneeded but it took me a few minutes so I don't wanna delete it...j
// const OPERATORS: &'static [&'static str] = &["+", "-", "*", "/", "%"];

fn addition(a: isize, b: isize) -> String {
    (a + b).to_string()
}

fn subtraction(a: isize, b: isize) -> String {
    (a - b).to_string()
}

fn multiplication(a: isize, b: isize) -> String {
    (a * b).to_string()
}

fn division(a: isize, b: isize) -> String {
    (a as f64 / b as f64).to_string()
}

fn modulo(a: isize, b: isize) -> String {
    (a % b).to_string()
}

fn main() {
    let matches = App::new("calc.rs")
                            .version("0.1")
                            .author("Ian S. Pringle <ian@dapringles.com>")
                            .about("Do math.")
                            .arg(Arg::with_name("verbose")
                                 .short("v")
                                 .long("verbose")
                                 .help("Sets verbose output"))
                            .arg(Arg::with_name("a")
                                 .help("Left number of equation")
                                 .required(true)
                                 .index(1))
                            .arg(Arg::with_name("OPERATOR")
                                 .help("Operator to evaluate numbers with")
                                 .required(true)
                                 .index(2))
                            .arg(Arg::with_name("b")
                                 .help("Right number of equation")
                                 .required(true)
                                 .index(3))
                            .get_matches();

    let a: isize = matches.value_of("a")
                                    .unwrap()
                                    .parse()
                                    .expect("Expected a number!");
    let b: isize = matches.value_of("b")
                                    .unwrap()
                                    .parse()
                                    .expect("Expected a number!");
    let op: String = matches.value_of("OPERATOR")
                                    .unwrap()
                                    .to_string();

    let result = match op.as_ref() {
        "+" => addition(a, b),
        "-" => subtraction(a, b),
        "x" => multiplication(a, b),
        "/" => division(a, b),
        "%" => modulo(a, b),
        _   => panic!("'{}' is an unsupported operator!", op),
    };

    match matches.occurrences_of("verbose") {
        0 => println!("{}", result),
        _ => println!("{} {} {} = {}", a, op, b, result),
    }
}
