pub use core::str::Chars;
use std::iter::Peekable;

const SPACE: char = ' ';
const SINGLE: char = '\'';
const DOUBLE: char = '"';
const BACKSLASH: char = '\\';
const GREATER_THAN: char = '>';

#[derive(Debug)]
pub enum StandardNamedStream {
    Unknown = -1,
    Output = 1,
    Error = 2,
}

impl From<u32> for StandardNamedStream {
    fn from(fd: u32) -> Self {
        match fd {
            1 => Self::Output,
            2 => Self::Error,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Redirect {
    pub stream_name: StandardNamedStream,
    pub path: String,
    pub append: bool,
}

pub struct ParsedLine {
    pub arguments: Vec<String>,
    pub redirects: Vec<Redirect>,
}

struct LineParser<'a> {
    chars: Peekable<Chars<'a>>,
    arguments: Vec<String>,
    redirects: Vec<Redirect>,
}

impl<'a> LineParser<'a> {
    fn new(line: &'a String) -> LineParser<'a> {
        LineParser {
            chars: line.chars().peekable(),
            arguments: Vec::new(),
            redirects: Vec::new(),
        }
    }

    fn parse(mut self) -> ParsedLine {
        while let Some(argument) = self.next_argument() {
            self.arguments.push(argument);
        }

        return ParsedLine {
            arguments: self.arguments,
            redirects: self.redirects,
        };
    }

    fn next_argument(&mut self) -> Option<String> {
        let mut builder: Vec<char> = Vec::new();

        while let Some(character) = self.chars.next() {
            match character {
                SPACE => {
                    if !builder.is_empty() {
                        let argument: String = builder.iter().collect();

                        return Some(argument);
                    }
                }
                SINGLE => {
                    while let Some(character) = self.chars.next() {
                        if character == SINGLE {
                            break;
                        }

                        builder.push(character);
                    }
                }
                DOUBLE => {
                    while let Some(character) = self.chars.next() {
                        match character {
                            DOUBLE => break,
                            BACKSLASH => self.backslash(&mut builder, true),
                            _ => builder.push(character),
                        }
                    }
                }
                BACKSLASH => self.backslash(&mut builder, false),
                GREATER_THAN => self.redirect(StandardNamedStream::Output),
                _ => {
                    if character.is_digit(10) && self.chars.peek() == Some(&GREATER_THAN) {
                        self.chars.next().unwrap();
                        self.redirect(character.to_digit(10).unwrap().into());
                    } else {
                        builder.push(character)
                    }
                }
            }
        }

        if !builder.is_empty() {
            let argument: String = builder.iter().collect();

            return Some(argument);
        }

        return None;
    }

    fn backslash(&mut self, builder: &mut Vec<char>, in_quote: bool) {
        if let Some(mut character) = self.chars.next() {
            if in_quote {
                if let Some(mapped) = LineParser::map_backslash_character(character) {
                    character = mapped;
                } else {
                    builder.push(BACKSLASH);
                }
            }

            builder.push(character);
        }
    }

    fn map_backslash_character(character: char) -> Option<char> {
        match character {
            BACKSLASH | DOUBLE => Some(character),
            _ => None,
        }
    }

    fn redirect(&mut self, stream_name: StandardNamedStream) {
        let append = self.chars.peek() == Some(&GREATER_THAN);
        if append {
            self.chars.next();
        }

        let path = self.next_argument().unwrap_or("".into());

        self.redirects.push(Redirect {
            stream_name,
            path,
            append
        });
    }
}

pub fn parse_argv(line: String) -> ParsedLine {
    LineParser::new(&line).parse()
}
