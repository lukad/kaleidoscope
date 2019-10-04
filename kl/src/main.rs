use std::{env, fs};

use kaleidoscope::parser::parse;

fn main() {
    let input_path = env::args().nth(1).expect("No input");
    let input = fs::read_to_string(input_path).expect("Could not read file");
    let res = parse(&input);
    println!("{:?}", res);
    // Parser::()
}
