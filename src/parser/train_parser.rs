use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use regex::Regex;

use crate::model::Settings;

pub fn parse_line(
    line: &str,
    word_map: &mut HashMap<String, i32>,
    history: &mut HashSet<String>,
    settings: &Settings,
) {
    let mut is_not_on = false;
    // Do the Notting
    let words = line.split(' ');
    for word in words {
        if !settings.ignored_words.contains(&word) {
            let word = {
                let tr = word.trim();
                if is_not_on && !settings.separators.contains(&tr) {
                    format!("[NOT]{}", tr)
                } else {
                    tr.to_string()
                }
            };
        
            if settings.notters.contains(&word.as_str()) {
                is_not_on = true;
            }
            if settings.separators.contains(&word.as_str()) {
                is_not_on = false;
            } else if !history.contains(&word) {
                history.insert(word.clone());
                let entry = word_map.entry(word).or_insert(0);
                *entry += 1;
            }
        }
    }
}

pub fn parse_file(
    filename: &Path,
    word_map: &mut HashMap<String, i32>,
    settings: &Settings,
) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let rd = BufReader::new(file);
    let re = Regex::new("[^a-zA-Z',;:\\. \n]").unwrap();
    let mut history: HashSet<String> = HashSet::new();

    for line_op in rd.lines() {
        let line = line_op?;
        let line = re.replace_all(&line, "");
        parse_line(&line, word_map, &mut history, settings);
    }

    Ok(())
}