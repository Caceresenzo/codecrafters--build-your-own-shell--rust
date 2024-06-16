use std::{collections::HashMap, process::exit};
#[allow(unused_imports)]
use std::{
    env,
    io::{self, Write},
    option::Option,
    sync::RwLock,
};

use lazy_static::lazy_static;

type BuiltinFunction = fn(Vec<&str>) -> ();
type BuiltinMap = HashMap<String, BuiltinFunction>;

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
    let arguments: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let program = arguments[0];

    match query(program) {
        Command::Builtin(builtin) => builtin(arguments),
        Command::None => println!("{}: command not found", program),
    }
}

enum Command {
    Builtin(BuiltinFunction),
    None,
}

fn query(program: &str) -> Command {
    if let Some(builtin) = BUILTINS.read().unwrap().get(program) {
        return Command::Builtin(*builtin);
    }

    return Command::None;
}

fn builtin_exit(_: Vec<&str>) {
    exit(0);
}

fn builtin_echo(arguments: Vec<&str>) {
    println!("{}", arguments[1..].join(" "));
}

fn builtin_type(arguments: Vec<&str>) {
    let program = arguments[1];

    match query(program) {
        Command::Builtin(_) => println!("{} is a shell builtin", program),
        Command::None => println!("{}: not found", program),
    }
}

lazy_static! {
    static ref BUILTINS: RwLock<BuiltinMap> = RwLock::new(HashMap::new());
}

fn main() {
    {
        let mut builtins = BUILTINS.write().unwrap();
        builtins.insert("exit".into(), builtin_exit);
        builtins.insert("echo".into(), builtin_echo);
        builtins.insert("type".into(), builtin_type);
    }

    loop {
        match read() {
            Some(line) => eval(line),
            None => break,
        }
    }
}
