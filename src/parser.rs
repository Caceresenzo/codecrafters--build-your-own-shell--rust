pub use core::str::Chars;

const SPACE: char = ' ';
const SINGLE: char = '\'';
const DOUBLE: char = '"';
const BACKSLASH: char = '\\';

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
                        match character {
                            DOUBLE => break,
                            BACKSLASH => self.backslash(true),
                            _ => self.builder.push(character),
                        }
                    }
                }
                BACKSLASH => self.backslash(false),
                _ => self.builder.push(character),
            }
        }

        if !self.builder.is_empty() {
            let arg: String = self.builder.iter().collect();
            argv.push(arg);
        }

        return argv;
    }

    fn backslash(&mut self, in_quote: bool) {
        if let Some(character) = self.chars.next() {
            if in_quote {
                self.builder.push(BACKSLASH);
            }

            self.builder.push(character);
        }
    }
}

pub fn parse_argv(line: String) -> Vec<String> {
    LineParser::new(&line).parse()
}
