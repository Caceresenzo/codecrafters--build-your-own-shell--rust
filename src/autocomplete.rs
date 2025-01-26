use std::{
    env, fs,
    io::{self, Write},
    vec::Vec,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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

    if let Ok(paths) = env::var("PATH") {
        for directory in paths.split(":") {
            if let Ok(entries) = fs::read_dir(directory) {
                for entry in entries {
                    let entry = entry.unwrap();
                    let name = entry.file_name().to_str().unwrap().to_string();

                    if !&name.starts_with(&*line) {
                        continue;
                    }

                    let metadata = entry.metadata().unwrap();
                    if !metadata.is_file() || metadata.permissions().mode() & 0o111 == 0 {
                        continue;
                    }

                    let candidate: String = name[line.len()..].into();
                    if !candidates.contains(&candidate) {
                        candidates.push(candidate);
                    }
                }
            }
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
