use std::{
    io::{self, Write},
    vec::Vec,
};

use crate::REGISTRY;

pub fn bell() {
    io::stdout().write(&[0x7]).unwrap();
    io::stdout().flush().unwrap();
}

pub enum AutocompleteResult {
    None,
    Found,
    More,
}

fn commit(line: &mut String, candidate: &String) {
    line.push_str(candidate);
    io::stdout().write(candidate.as_bytes()).unwrap();

    line.push(' ');
    io::stdout().write(&[b' ']).unwrap();

    io::stdout().flush().unwrap();
}

pub fn autocomplete(line: &mut String) -> AutocompleteResult {
    let mut candidates: Vec<String> = Vec::new();

    for key in REGISTRY.read().unwrap().keys() {
        if key.starts_with(&*line) {
            let candidate = &key[line.len()..];
            candidates.push(candidate.to_string());
        }
    }

    if candidates.is_empty() {
        return AutocompleteResult::None;
    } else if candidates.len() == 1 {
        commit(line, &candidates[0]);
        return AutocompleteResult::Found;
    }

    todo!("more");
    // return AutocompleteResult::More;
}
