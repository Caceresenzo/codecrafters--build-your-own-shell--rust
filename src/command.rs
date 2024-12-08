use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{BuiltinFunction, REGISTRY};

pub enum ShellCommand {
    Builtin(BuiltinFunction),
    Executable(PathBuf),
    None,
}

pub fn query(program: &String) -> ShellCommand {
    if let Some(builtin) = REGISTRY.read().unwrap().get(program.as_str()) {
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
