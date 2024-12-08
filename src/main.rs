use std::{
    io::{self, Write},
    option::Option,
    process::{Command, Stdio},
};

use shell_starter_rust::{query, register_default_builtins, ShellCommand, parse_argv};

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
    let arguments: Vec<String> = parse_argv(line);
    if arguments.is_empty() {
        return
    }

    let program = &arguments[0];
    match query(program) {
        ShellCommand::Builtin(builtin) => builtin(arguments),
        ShellCommand::Executable(path) => {
            Command::new(path)
                .args(&arguments[1..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("failed to execute process");
        }
        ShellCommand::None => println!("{}: command not found", program),
    }
}

fn main() {
    register_default_builtins();

    loop {
        match read() {
            Some(line) => eval(line),
            None => break,
        }
    }
}
