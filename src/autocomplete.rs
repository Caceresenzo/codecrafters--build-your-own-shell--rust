use std::{
    env, fs,
    io::{self, Write},
    vec::Vec,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::Shell;

pub fn prompt() {
    io::stdout().write("$ ".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn bell() {
    io::stdout().write(&[0x7]).unwrap();
    io::stdout().flush().unwrap();
}

pub enum AutocompleteResult {
    None,
    Found,
    More,
}

fn commit(line: &mut String, candidate: &String, has_more: bool) {
    line.push_str(candidate);
    io::stdout().write(candidate.as_bytes()).unwrap();

    if !has_more {
        line.push(' ');
        io::stdout().write(&[b' ']).unwrap();
    }

    io::stdout().flush().unwrap();
}

fn find_shared_prefix(candidates: &Vec<String>) -> String {
    let first = candidates.first().unwrap();

    if first.is_empty() {
        return "".into();
    }

    let mut end = 1;
    for i in 1..first.len() + 1 {
        end = i; // ugly

        let mut one_is_not_matching = false;

        for candidate in candidates.iter().skip(1) {
            if first[..end] != candidate[..end] {
                one_is_not_matching = true;
                break;
            }
        }

        if one_is_not_matching {
            end -= 1;
            break;
        }
    }

    return first[..end].to_string();
}

pub fn autocomplete(shell: &Shell, line: &mut String, bell_rang: bool) -> AutocompleteResult {
    let mut candidates: Vec<String> = Vec::new();

    for key in shell.builtins.keys() {
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
        commit(line, &candidates[0], false);
        return AutocompleteResult::Found;
    }

    candidates.sort_by(|a, b| {
        if a.len() < b.len() {
            std::cmp::Ordering::Less
        } else if a.len() > b.len() {
            std::cmp::Ordering::Greater
        } else {
            a.partial_cmp(b).unwrap()
        }
    });

    let prefix = find_shared_prefix(&candidates);
    if !prefix.is_empty() {
        commit(line, &prefix, true);
        return AutocompleteResult::Found;
    }

    if bell_rang {
        io::stdout().write(&[b'\n']).unwrap();

        for (index, candidate) in candidates.iter().enumerate() {
            if index != 0 {
                io::stdout().write("  ".as_bytes()).unwrap();
            }

            io::stdout().write(&line.as_bytes()).unwrap();
            io::stdout().write(&candidate.as_bytes()).unwrap();
        }

        io::stdout().write(&[b'\n']).unwrap();
        prompt();
        io::stdout().write(&line.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }

    return AutocompleteResult::More;
}
