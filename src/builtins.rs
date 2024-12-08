use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    env,
    option::Option,
    path::{Path, PathBuf},
    process::exit,
    sync::RwLock,
};

use crate::{query, ShellCommand};

pub type BuiltinFunction = fn(Vec<&str>) -> ();
pub type BuiltinMap = HashMap<String, BuiltinFunction>;

lazy_static! {
    pub static ref REGISTRY: RwLock<BuiltinMap> = RwLock::new(HashMap::new());
}

pub fn builtin_exit(_: Vec<&str>) {
    exit(0);
}

pub fn builtin_echo(arguments: Vec<&str>) {
    println!("{}", arguments[1..].join(" "));
}

pub fn builtin_type(arguments: Vec<&str>) {
    let program = arguments[1];

    match query(program) {
        ShellCommand::Builtin(_) => println!("{} is a shell builtin", program),
        ShellCommand::Executable(path) => println!("{} is {}", program, path.to_str().unwrap()),
        ShellCommand::None => println!("{}: not found", program),
    }
}

pub fn builtin_pwd(_: Vec<&str>) {
    match env::current_dir() {
        Err(e) => println!("pwd: {}", e),
        Ok(path) => println!("{}", path.to_str().unwrap()),
    }
}

pub fn builtin_cd(arguments: Vec<&str>) {
    let mut absolute_path: Option<PathBuf> = None;

    let path = arguments[1];
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
            Err(_) => println!("cd: $HOME not set"),
        }
    }

    if let Some(path_buf) = absolute_path {
        if let Err(_) = env::set_current_dir(path_buf) {
            println!("cd: {}: No such file or directory", path);
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
