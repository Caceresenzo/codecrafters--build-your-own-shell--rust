use crate::*;
use std::{
    env,
    path::{Path, PathBuf},
};
pub enum ShellCommand {
    Builtin(BuiltinFunction),
    Executable(PathBuf),
    None,
}

pub struct Shell {
    pub builtins: BuiltinMap,
    pub history: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {
        let mut builtins = BuiltinMap::new();
        builtins.insert("exit".into(), builtin_exit);
        builtins.insert("echo".into(), builtin_echo);
        builtins.insert("type".into(), builtin_type);
        builtins.insert("pwd".into(), builtin_pwd);
        builtins.insert("cd".into(), builtin_cd);
        builtins.insert("history".into(), builtin_history);

        Shell {
            builtins,
            history: Vec::new(),
        }
    }

    pub fn query(&self, program: &String) -> ShellCommand {
        if let Some(builtin) = self.builtins.get(program.as_str()) {
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

    pub fn read_history(&mut self, path: &String) {
        if let Ok(lines) = std::fs::read_to_string(path) {
            self.history.extend(lines.lines().map(String::from));
        }
    }

    pub fn write_history(&self, path: &String) {
        std::fs::write(path, self.history.join("\n") + "\n").unwrap();
    }
}
