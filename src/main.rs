use std::io::{self, Read, Write};
use std::os::unix::io::RawFd;

use termios::{tcsetattr, Termios};

use shell_starter_rust::{
    autocomplete, bell, parse_argv, prompt, register_default_builtins, run_pipeline, run_single,
    AutocompleteResult,
};

enum ReadResult {
    Quit,
    Empty,
    Content(String),
}

fn read() -> ReadResult {
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
            '\t' => match autocomplete(&mut line, bell_rang) {
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
                let _ = io::stdin().read(&mut buffer); // 'A' or 'B' or 'C' or 'D'
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

fn eval(line: String) {
    let commands = parse_argv(line);

    match commands.len() {
        0 => return,
        1 => run_single(&commands[0]),
        _ => run_pipeline(commands),
    }
}

fn main() {
    register_default_builtins();

    loop {
        match read() {
            ReadResult::Quit => break,
            ReadResult::Empty => continue,
            ReadResult::Content(line) => eval(line),
        }
    }
}
