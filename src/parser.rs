pub use core::str::Chars;

const SPACE: char = ' ';
const SINGLE: char = '\'';
const DOUBLE: char = '"';

struct LineParser<'a> {
    chars: Chars<'a>,
    builder: Vec<char>,
}

impl<'a> LineParser<'a> {
    fn new(line: &'a String) -> LineParser<'a> {
        LineParser {
            chars: line.chars(),
            builder: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<String> {
        let mut argv: Vec<String> = Vec::new();

        while let Some(character) = self.chars.next() {
            match character {
                SPACE => {
                    if !self.builder.is_empty() {
                        let arg: String = self.builder.iter().collect();
                        argv.push(arg);

                        self.builder.clear();
                    }
                }
                SINGLE => {
                    while let Some(character) = self.chars.next() {
                        if character == SINGLE {
                            break;
                        }

                        self.builder.push(character);
                    }
                }
                DOUBLE => {
                    while let Some(character) = self.chars.next() {
                        if character == DOUBLE {
                            break;
                        }

                        self.builder.push(character);
                    }
                }
                _ => {
                    self.builder.push(character);
                }
            }
        }

        if !self.builder.is_empty() {
            let arg: String = self.builder.iter().collect();
            argv.push(arg);
        }

        return argv;
    }
}

pub fn parse_argv(line: String) -> Vec<String> {
    LineParser::new(&line).parse()
}
