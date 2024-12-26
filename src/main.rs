use std::{
    fs::File,
    io::{self, Write},
    option::Option,
    process::{Command, Stdio},
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use shell_starter_rust::{
    parse_argv, query, register_default_builtins, RedirectStreams, ShellCommand,
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
    let parsed_line = parse_argv(line);

    let arguments = parsed_line.arguments;
    if arguments.is_empty() {
        return;
    }

    let mut redirected_streams = RedirectStreams::new(parsed_line.redirects).unwrap();

    let program = &arguments[0];
    match query(program) {
        ShellCommand::Builtin(builtin) => builtin(arguments, &mut redirected_streams),
        ShellCommand::Executable(path) => {
            let mut command = Command::new(path);

            #[cfg(unix)]
            command.arg0(&arguments[0]);

            command
                .args(&arguments[1..])
                .stdin(Stdio::inherit())
                .stdout(if let Some(file) = redirected_streams.output {
                    From::<File>::from(file)
                } else {
                    Stdio::inherit()
                })
                .stderr(if let Some(file) = redirected_streams.error {
                    From::<File>::from(file)
                } else {
                    Stdio::inherit()
                })
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
