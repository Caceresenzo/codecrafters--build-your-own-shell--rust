use clap::Parser;
use std::io::{self, Read, Write};
use std::os::unix::io::RawFd;
use std::process::exit;

use termios::{tcsetattr, Termios};

use shell_starter_rust::{
    autocomplete, bell, parse_argv, prompt, run_pipeline, run_single, AutocompleteResult,
    RedirectStreams, Shell, ShellCommand,
};

enum ReadResult {
    Quit,
    Empty,
    Content(String),
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long, num_args=1..=100)]
    builtin: Vec<String>,
}

const UP: u8 = b'A';

fn change_line(line: &mut String, new_content: &String) {
    let mut backspaces = String::new();
    let mut spaces = String::new();
    for _ in 0..line.len() {
        backspaces.push('\u{8}');
        spaces.push(' ');
    }

    io::stdout().write(backspaces.as_bytes()).unwrap();
    io::stdout().write(spaces.as_bytes()).unwrap();
    io::stdout().write(backspaces.as_bytes()).unwrap();

    io::stdout().write(new_content.as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    line.clear();
    line.push_str(new_content);
}

fn read(shell: &Shell) -> ReadResult {
    prompt();

    let stdin_fd: RawFd = 0;
    let previous = Termios::from_fd(stdin_fd).unwrap();

    let mut new = previous.clone();
    new.c_iflag &= termios::IGNCR;
    new.c_lflag ^= termios::ICANON;
    new.c_lflag ^= termios::ECHO;
    new.c_cc[termios::VMIN] = 1;
    new.c_cc[termios::VTIME] = 0;

    tcsetattr(stdin_fd, termios::TCSANOW, &new).unwrap();

    let history_len = shell.history.len();
    let mut history_position = history_len;

    let mut line = String::new();
    let mut buffer = [0u8];

    let mut bell_rang = false;

    let result: ReadResult;
    loop {
        match io::stdin().read(&mut buffer) {
            Err(_) | Ok(0) => {
                result = ReadResult::Quit;
                break;
            }
            Ok(_) => {}
        }

        let character = buffer[0] as char;
        match character {
            '\u{4}' => {
                result = ReadResult::Quit;
                break;
            }
            '\r' | '\n' => {
                io::stdout().write("\r\n".as_bytes()).unwrap();
                io::stdout().flush().unwrap();

                result = if line.len() == 0 {
                    ReadResult::Empty
                } else {
                    ReadResult::Content(line)
                };
                break;
            }
            '\t' => match autocomplete(shell, &mut line, bell_rang) {
                AutocompleteResult::None => {
                    bell_rang = false;
                    bell();
                }
                AutocompleteResult::Found => {
                    bell_rang = false;
                }
                AutocompleteResult::More => {
                    bell_rang = true;
                    bell();
                }
            },
            '\u{1b}' => {
                let _ = io::stdin().read(&mut buffer); // '['

                if let Ok(_) = io::stdin().read(&mut buffer) {
                    let direction = buffer[0];

                    if direction == UP && history_position != 0 {
                        history_position -= 1;
                        change_line(&mut line, &shell.history[history_position]);
                    }
                }
            }
            '\u{7f}' => {
                if line.len() != 0 {
                    line.pop();
                    io::stdout().write("\u{8} \u{8}".as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
            }
            _ => {
                io::stdout().write(&buffer).unwrap();
                io::stdout().flush().unwrap();
                line.push(character);
            }
        }
    }

    tcsetattr(stdin_fd, termios::TCSANOW, &previous).unwrap();

    return result;
}

fn eval(shell: &mut Shell, line: String) {
    shell.history.push(line.clone());

    let commands = parse_argv(line);

    match commands.len() {
        0 => return,
        1 => run_single(shell, &commands[0]),
        _ => run_pipeline(shell, commands),
    }
}

fn main() {
    let mut shell = Shell::new();
    // shell.history.push("111".to_string());
    // shell.history.push("222".to_string());
    // shell.history.push("333".to_string());

    let args = Args::parse();
    if !args.builtin.is_empty() {
        let program = &args.builtin[0];
        match shell.query(program) {
            ShellCommand::Builtin(builtin) => {
                let mut standard = RedirectStreams::standard();
                builtin(&mut shell, &args.builtin, &mut standard);
                exit(0);
            }
            _ => {
                eprintln!("{}: command not found", program);
                exit(1);
            }
        }
    }

    loop {
        match read(&shell) {
            ReadResult::Quit => break,
            ReadResult::Empty => continue,
            ReadResult::Content(line) => eval(&mut shell, line),
        }
    }
}
