use crate::{RedirectStreams, Shell, ShellCommand};
use std::{
    collections::HashMap,
    env,
    option::Option,
    path::{Path, PathBuf},
    process::exit,
};

pub type BuiltinFunction = fn(&mut Shell, &Vec<String>, &mut RedirectStreams) -> ();
pub type BuiltinMap = HashMap<String, BuiltinFunction>;

pub fn builtin_exit(_: &mut Shell, _: &Vec<String>, _: &mut RedirectStreams) {
    exit(0);
}

pub fn builtin_echo(_: &mut Shell, arguments: &Vec<String>, io: &mut RedirectStreams) {
    io.println(format!("{}", arguments[1..].join(" ")).as_str());
}

pub fn builtin_type(shell: &mut Shell, arguments: &Vec<String>, io: &mut RedirectStreams) {
    let program = &arguments[1];

    match shell.query(program) {
        ShellCommand::Builtin(_) => io.println(format!("{} is a shell builtin", program).as_str()),
        ShellCommand::Executable(path) => {
            io.println(format!("{} is {}", program, path.to_str().unwrap()).as_str())
        }
        ShellCommand::None => io.println(format!("{}: not found", program).as_str()),
    }
}

pub fn builtin_pwd(_: &mut Shell, _: &Vec<String>, io: &mut RedirectStreams) {
    match env::current_dir() {
        Err(e) => io.println_error(format!("pwd: {}", e).as_str()),
        Ok(path) => io.println(format!("{}", path.to_str().unwrap()).as_str()),
    }
}

pub fn builtin_cd(_: &mut Shell, arguments: &Vec<String>, io: &mut RedirectStreams) {
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
}

pub fn builtin_history(shell: &mut Shell, _: &Vec<String>, _: &mut RedirectStreams) {
    for (index, command) in shell.history.iter().enumerate() {
        println!("{:5}  {}", index + 1, command);
    }
}
