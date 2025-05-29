use std::{
    env::current_exe,
    fs::File,
    process::{exit, Child, Command, Stdio},
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use crate::{ParsedLine, RedirectStreams, Shell, ShellCommand};
use fork::{fork, waitpid, Fork};

pub fn run_single(shell: &mut Shell, parsed_line: &ParsedLine) {
    let arguments = &parsed_line.arguments;
    if arguments.is_empty() {
        return;
    }

    let mut redirected_streams = RedirectStreams::new(&parsed_line.redirects).unwrap();

    let program = &arguments[0];
    match shell.query(program) {
        ShellCommand::Builtin(builtin) => builtin(shell, &arguments, &mut redirected_streams),
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
        ShellCommand::None => eprintln!("{}: command not found", program),
    }
}

pub fn run_pipeline(shell: &mut Shell, commands: Vec<ParsedLine>) {
    match fork().unwrap() {
        Fork::Parent(child) => waitpid(child).unwrap(),
        Fork::Child => _do_run_pipeline(shell, commands),
    }
}

fn _do_run_pipeline(shell: &mut Shell, parsed_lines: Vec<ParsedLine>) {
    let mut childs: Vec<Box<Child>> = Vec::new();

    for (index, parsed_line) in parsed_lines.iter().enumerate() {
        let redirected_streams = RedirectStreams::new(&parsed_line.redirects).unwrap();

        let is_first = index == 0;
        let is_last = index == parsed_lines.len() - 1;

        let program = &parsed_line.arguments[0];
        match shell.query(program) {
            ShellCommand::Builtin(_) => {
                let mut command = Command::new(current_exe().unwrap());

                if is_first {
                    command.stdin(Stdio::inherit());
                } else {
                    command.stdin(childs[index - 1].stdout.take().unwrap());
                }

                let mut arguments: Vec<String> = Vec::new();
                arguments.push("--builtin".into());
                arguments.extend(parsed_line.arguments.clone());

                let child = command
                    .args(arguments)
                    .stdout(if let Some(file) = redirected_streams.output {
                        From::<File>::from(file)
                    } else if is_last {
                        Stdio::inherit()
                    } else {
                        Stdio::piped()
                    })
                    .stderr(if let Some(file) = redirected_streams.error {
                        From::<File>::from(file)
                    } else {
                        Stdio::inherit()
                    })
                    .spawn()
                    .expect("failed to execute process");

                childs.push(Box::new(child));
            }
            ShellCommand::Executable(path) => {
                let mut command = Command::new(path);

                #[cfg(unix)]
                command.arg0(&parsed_line.arguments[0]);

                if is_first {
                    command.stdin(Stdio::inherit());
                } else {
                    command.stdin(childs[index - 1].stdout.take().unwrap());
                }

                let child = command
                    .args(&parsed_line.arguments[1..])
                    .stdout(if let Some(file) = redirected_streams.output {
                        From::<File>::from(file)
                    } else if is_last {
                        Stdio::inherit()
                    } else {
                        Stdio::piped()
                    })
                    .stderr(if let Some(file) = redirected_streams.error {
                        From::<File>::from(file)
                    } else {
                        Stdio::inherit()
                    })
                    .spawn()
                    .expect("failed to execute process");

                childs.push(Box::new(child));
            }
            ShellCommand::None => eprintln!("{}: command not found", program),
        }
    }

    childs.last_mut().unwrap().wait().unwrap();

    exit(0);
}
