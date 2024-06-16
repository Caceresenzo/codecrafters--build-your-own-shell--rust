#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin: io::Stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let program = &input[..input.len() - 1];
    println!("{}: command not found", program);
}
