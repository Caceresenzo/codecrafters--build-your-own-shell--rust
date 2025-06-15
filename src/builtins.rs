use crate::{RedirectStreams, Shell, ShellCommand};
use std::{
    collections::HashMap,
    env,
    option::Option,
    path::{Path, PathBuf},
};

pub type BuiltinFunction = fn(&mut Shell, &Vec<String>, &mut RedirectStreams) -> Option<i32>;
pub type BuiltinMap = HashMap<String, BuiltinFunction>;

pub fn builtin_exit(_: &mut Shell, _: &Vec<String>, _: &mut RedirectStreams) -> Option<i32> {
    Some(0)
}

pub fn builtin_echo(
    _: &mut Shell,
    arguments: &Vec<String>,
    io: &mut RedirectStreams,
) -> Option<i32> {
    io.println(format!("{}", arguments[1..].join(" ")).as_str());

    None
}

pub fn builtin_type(
    shell: &mut Shell,
    arguments: &Vec<String>,
    io: &mut RedirectStreams,
) -> Option<i32> {
    let program = &arguments[1];

    match shell.query(program) {
        ShellCommand::Builtin(_) => io.println(format!("{} is a shell builtin", program).as_str()),
        ShellCommand::Executable(path) => {
            io.println(format!("{} is {}", program, path.to_str().unwrap()).as_str())
        }
        ShellCommand::None => io.println(format!("{}: not found", program).as_str()),
    }

    None
}

pub fn builtin_pwd(_: &mut Shell, _: &Vec<String>, io: &mut RedirectStreams) -> Option<i32> {
    match env::current_dir() {
        Err(e) => io.println_error(format!("pwd: {}", e).as_str()),
        Ok(path) => io.println(format!("{}", path.to_str().unwrap()).as_str()),
    }

    None
}

pub fn builtin_cd(_: &mut Shell, arguments: &Vec<String>, io: &mut RedirectStreams) -> Option<i32> {
    let mut absolute_path: Option<PathBuf> = None;

    let path = &arguments[1];
    if path.starts_with("/") {
        absolute_path = Some(Path::new(&path).into());
    } else if path.starts_with(".") {
        if let Ok(current_path) = env::current_dir() {
            absolute_path = Some(Path::new(&current_path).join(path));
        }
    } else if path.starts_with("~") {
        match env::var("HOME") {
            Ok(home) => {
                absolute_path = Some(Path::new(&home).join(format!("./{}", &path[1..])));
            }
            Err(_) => io.println_error("cd: $HOME not set"),
        }
    }

    if let Some(path_buf) = absolute_path {
        if let Err(_) = env::set_current_dir(path_buf) {
            io.println_error(format!("cd: {}: No such file or directory", path).as_str());
        }
    }

    None
}

fn print_history(start: usize, shell: &Shell, io: &mut RedirectStreams) {
    for (index, command) in shell.history.iter().skip(start).enumerate() {
        io.println(format!("{:5}  {}", index + 1, command).as_str());
    }
}

pub fn builtin_history(
    shell: &mut Shell,
    arguments: &Vec<String>,
    io: &mut RedirectStreams,
) -> Option<i32> {
    let first = if arguments.len() > 1 {
        Some(&arguments[1])
    } else {
        None
    };

    match first {
        Some(arg) if arg == "-r" => shell.read_history(&arguments[2]),
        Some(arg) if arg == "-w" => shell.write_history(&arguments[2]),
        Some(arg) if arg == "-a" => shell.append_history(&arguments[2]),
        Some(value) if value.chars().all(char::is_numeric) => {
            let start = shell.history.len() - value.parse::<usize>().unwrap();
            print_history(start, shell, io);
        }
        None => print_history(0, shell, io),
        _ => io.println_error("history: invalid parameter"),
    }

    None
}
