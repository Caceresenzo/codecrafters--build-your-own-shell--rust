#[allow(unused_imports)]
use std::{
    io::{self, Write},
    option::Option
};

fn read() -> Option<String> {
    let stdin: io::Stdin = io::stdin();
    let mut input = String::new();
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let result: Option<String> = match stdin.read_line(&mut input) {
            Err(_) => None,
            Ok(size) if size == 0 => None,
            Ok(_) => Some(input.trim().into()),
        };

        if let Some(ref line) = result {
            if line.len() != 0 {
                return result;
            }
        }

        if let None = result {
            return result;
        }
    }
}

fn eval(line: String) {
    let program = line;
    println!("{}: command not found", program);
}

fn main() {
    loop {
        match read() {
            Some(line) => eval(line),
            None => break,
        }
    }
}
