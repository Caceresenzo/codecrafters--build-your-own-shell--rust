use std::{
    fs::File,
    io::{self, Write},
};

use crate::{Redirect, StandardNamedStream};

pub struct RedirectStreams {
    pub output: Option<File>,
    pub error: Option<File>,
}

impl RedirectStreams {
    pub fn new(redirects: &Vec<Redirect>) -> Result<RedirectStreams, io::Error> {
        let mut output: Option<File> = None;
        let mut error: Option<File> = None;

        for redirect in redirects {
            let file = File::options()
                .create(true)
                .truncate(!redirect.append)
                .append(redirect.append)
                .write(true)
                .open(redirect.path.clone())?;

            match redirect.stream_name {
                StandardNamedStream::Output => output = Some(file),
                StandardNamedStream::Error => error = Some(file),
                StandardNamedStream::Unknown => {}
            };
        }

        return Ok(RedirectStreams { output, error });
    }

    pub fn println(&mut self, message: &str) {
        if let Some(file) = &mut self.output {
            writeln!(file, "{message}").expect("could not print to redirected stdout");
        } else {
            println!("{message}");
        }
    }

    pub fn println_error(&mut self, message: &str) {
        if let Some(file) = &mut self.error {
            writeln!(file, "{message}").expect("could not print to redirected stderr");
        } else {
            eprintln!("{message}");
        }
    }
}
