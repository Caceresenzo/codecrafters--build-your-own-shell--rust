use crate::{query, RedirectStreams, ShellCommand};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    env,
    option::Option,
    path::{Path, PathBuf},
    process::exit,
    sync::RwLock,
};

pub type BuiltinFunction = fn(Vec<String>, &mut RedirectStreams) -> ();
pub type BuiltinMap = HashMap<String, BuiltinFunction>;

lazy_static! {
    pub static ref REGISTRY: RwLock<BuiltinMap> = RwLock::new(HashMap::new());
}

pub fn builtin_exit(_: Vec<String>, _: &mut RedirectStreams) {
    exit(0);
}

pub fn builtin_echo(arguments: Vec<String>, io: &mut RedirectStreams) {
    io.println(format!("{}", arguments[1..].join(" ")).as_str());
}

pub fn builtin_type(arguments: Vec<String>, io: &mut RedirectStreams) {
    let program = &arguments[1];

    match query(program) {
        ShellCommand::Builtin(_) => io.println(format!("{} is a shell builtin", program).as_str()),
        ShellCommand::Executable(path) => {
            io.println(format!("{} is {}", program, path.to_str().unwrap()).as_str())
        }
        ShellCommand::None => io.println(format!("{}: not found", program).as_str()),
    }
}

pub fn builtin_pwd(_: Vec<String>, io: &mut RedirectStreams) {
    match env::current_dir() {
        Err(e) => io.println_error(format!("pwd: {}", e).as_str()),
        Ok(path) => io.println(format!("{}", path.to_str().unwrap()).as_str()),
    }
}

pub fn builtin_cd(arguments: Vec<String>, io: &mut RedirectStreams) {
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

pub fn register_default_builtins() {
    let mut builtins = REGISTRY.write().unwrap();
    builtins.insert("exit".into(), builtin_exit);
    builtins.insert("echo".into(), builtin_echo);
    builtins.insert("type".into(), builtin_type);
    builtins.insert("pwd".into(), builtin_pwd);
    builtins.insert("cd".into(), builtin_cd);
}
