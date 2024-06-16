use std::{collections::HashMap, process::exit};
#[allow(unused_imports)]
use std::{
    io::{self, Write},
    option::Option
};

type BuiltinMap = HashMap<String, fn(Vec<&str>) -> ()>;

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

fn eval(line: String, builtins: &BuiltinMap) {
    let arguments: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let program = arguments[0];

    if let Some(builtin) = builtins.get(program) {
        builtin(arguments);
        return;
    }

    println!("{}: command not found", program);
}

fn builtin_exit(_: Vec<&str>) {
    exit(0);
}

fn builtin_echo(arguments: Vec<&str>) {
    println!("{}", arguments[1..].join(" "));
}

fn main() {
    let mut builtins: BuiltinMap = HashMap::new();
    builtins.insert("exit".into(), builtin_exit);
    builtins.insert("echo".into(), builtin_echo);

    loop {
        match read() {
            Some(line) => eval(line, &builtins),
            None => break,
        }
    }
}
