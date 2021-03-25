
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use regex::Regex;

use crate::model::Settings;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn test_line(
    line: &str,
    word_map: &HashMap<String, f32>,
    settings: &Settings,
) -> bool {
    let mut history = HashSet::new();
    let mut is_not_on = false;
    let mut sent: f32 = 0.0;
    let parts = line.split('\t').collect::<Vec<&str>>();
    if parts.len() < 2 {
        return true;
    }
    let words = parts[0].split(' ');
    for word in words {
        if !settings.ignored_words.contains(&word) {
            let word = {
                let tr = word.trim();
                tr.to_string()
            };
        
            if settings.notters.contains(&word.as_str()) {
                is_not_on = true;
            }
            if settings.separators.contains(&word.as_str()) {
                is_not_on = false;
            } else if !history.contains(&word) {
                history.insert(word.clone());
                if is_not_on {
                    sent -= word_map.get(&word).unwrap_or(&0.0);
                } else {
                    sent += word_map.get(&word).unwrap_or(&0.0);
                }
            }
        }
    }

    if (sent * (parse_input!(parts[1], i32) * 2 - 1) as f32) < 0.0 {
        println!("{} {}", line, sent);
    }
    sent * (parse_input!(parts[1], i32) * 2 - 1) as f32 > 0.0
}

pub fn test_file(
    filename: &Path,
    word_map: &HashMap<String, f32>,
    settings: &Settings,
) -> std::io::Result<(i32, i32)> {
    let file = File::open(filename)?;
    let rd = BufReader::new(file);
    let re = Regex::new("[^a-zA-Z0-9',;:\\.\t \n]").unwrap();

    let mut n = 0;
    let mut correct = 0;
    for line_op in rd.lines() {
        n += 1;
        let line = line_op?;
        let line = re.replace_all(&line, "");
        if test_line(&line, word_map, settings) {
            correct += 1;
        }
    }

    Ok((n, correct))
}