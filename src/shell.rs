use crate::*;
use std::{
    env,
    path::{Path, PathBuf},
    io::Write,
};
pub enum ShellCommand {
    Builtin(BuiltinFunction),
    Executable(PathBuf),
    None,
}

pub struct Shell {
    pub builtins: BuiltinMap,
    pub history: Vec<String>,
    last_history_append_index: usize,
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
            last_history_append_index: 0,
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
         self.do_write_history(&self.history, path, false);
    }

    pub fn append_history(&mut self, path: &String) {
        self.do_write_history(&self.history[self.last_history_append_index..], path, true);

        self.last_history_append_index = self.history.len();
    }

    fn do_write_history(&self, lines: &[String], path: &String, append: bool) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(append)
            .open(path)
            .unwrap();

        for line in lines.iter() {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
