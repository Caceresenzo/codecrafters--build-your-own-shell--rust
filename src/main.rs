use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    option::Option,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    sync::RwLock,
};

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

enum ShellCommand {
    Builtin(BuiltinFunction),
    Executable(PathBuf),
    None,
}

fn query(program: &str) -> ShellCommand {
    if let Some(builtin) = BUILTINS.read().unwrap().get(program) {
        return ShellCommand::Builtin(*builtin);
    }

    if let Ok(paths) = env::var("PATH") {
        for directory in paths.split(":") {
            let path = Path::new(directory).join(program);

            if path.exists() {
                return ShellCommand::Executable(path);
            }
        }
    }

    return ShellCommand::None;
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
        ShellCommand::Builtin(_) => println!("{} is a shell builtin", program),
        ShellCommand::Executable(path) => println!("{} is {}", program, path.to_str().unwrap()),
        ShellCommand::None => println!("{}: not found", program),
    }
}

fn builtin_pwd(_: Vec<&str>) {
    match env::current_dir() {
        Err(e) => println!("pwd: {}", e),
        Ok(path) => println!("{}", path.to_str().unwrap()),
    }
}

fn builtin_cd(arguments: Vec<&str>) {
    let mut absolute_path: Option<PathBuf> = None;

    let path = arguments[1];
    if path.starts_with("/") {
        absolute_path = Some(Path::new(&path).into());
    } else if path.starts_with(".") {
        if let Ok(current_path) = env::current_dir() {
            absolute_path = Some(Path::new(&current_path).join(path));
        }
    }

    if let Some(path_buf) = absolute_path {
        if let Err(_) = env::set_current_dir(path_buf) {
            println!("cd: {}: No such file or directory", path);
        }
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
        builtins.insert("pwd".into(), builtin_pwd);
        builtins.insert("cd".into(), builtin_cd);
    }

    loop {
        match read() {
            Some(line) => eval(line),
            None => break,
        }
    }
}
