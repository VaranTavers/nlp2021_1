use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

fn parse_line(line: &str, word_map: &mut HashMap<String, i32>, history: &mut HashSet<String>) {
    let mut is_not_on = false;
    let ignored_words = vec!["\"", "", "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "should", "now"];
    let notters = vec!["no", "not", "doesn't", "don't", "neither", "nor", "shouldn't", "couldn't", "wouldn't", "won't", "isn't", "aren't", "nobody", "none", "nothing"];
    let separators = vec![",", ":", ";", "."];
    // Do the Notting
    let words = line.split(' ');
    for word in words {
        if !ignored_words.contains(&word) {
            let word = {
                let tr = word.trim();
                if is_not_on && !separators.contains(&tr) {
                    format!("[NOT]{}", tr)
                } else {
                    tr.to_string()
                }
            };
        
            if notters.contains(&word.as_str()) {
                is_not_on = true;
            }
            if separators.contains(&word.as_str()) {
                is_not_on = false;
            } else if !history.contains(&word) {
                history.insert(word.clone());
                let entry = word_map.entry(word).or_insert(0);
                *entry += 1;
            }
        }
    }
}

fn parse_file(filename: &Path, word_map: &mut HashMap<String, i32>) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let rd = BufReader::new(file);
    let re = Regex::new("[^a-zA-Z',;:\\. \n]").unwrap();
    let mut history: HashSet<String> = HashSet::new();

    for line_op in rd.lines() {
        let line = line_op?;
        let line = re.replace_all(&line, "");
        parse_line(&line, word_map, &mut history);
    }

    Ok(())
}

fn get_map_from_folder(folder_path: &Path) -> std::io::Result<HashMap<String, i32>> {
    let mut ret: HashMap<String, i32> = HashMap::new();
    
    let folder = std::fs::read_dir(folder_path)?;
    for entry in folder {
        parse_file(&(entry.unwrap().path()), &mut ret)?;
    }

    Ok(ret)
}

fn denot_word(word: &str) -> &str {
    if word.contains("[NOT]") {
        &word[5..]
    } else {
        word
    }
}

fn main() -> std::io::Result<()> {
    let neg_map = get_map_from_folder(Path::new("./neg"))?;
    println!("The negative reviews have been parsed!");
    println!("Not impr: {} Impr: {}", neg_map.get("[NOT]impressed").unwrap_or(&0), neg_map.get("impressed").unwrap_or(&0));
    let pos_map = get_map_from_folder(Path::new("./pos"))?;
    println!("The positive reviews have been parsed!");
    // Results in (positivity in [-1, 1], and predicted accuracy [0, 1])
    let mut res_map: HashMap<String, (u64, u64)> = HashMap::new();

    for k in pos_map.keys() {
        let k = denot_word(k);
        let notk = format!("[NOT]{}", k);

        let pos = pos_map.get(k).unwrap_or(&0) + neg_map.get(&notk).unwrap_or(&0);
        let neg = neg_map.get(k).unwrap_or(&0) + pos_map.get(&notk).unwrap_or(&0); 
        res_map.insert(k.to_string(), (pos as u64, neg as u64));
    }

    for k in neg_map.keys() {
        let k = denot_word(k);
        if !res_map.contains_key(k) {
            let notk = format!("[NOT]{}", k);

            let pos = pos_map.get(k).unwrap_or(&0) + neg_map.get(&notk).unwrap_or(&0);
            let neg = neg_map.get(k).unwrap_or(&0) + pos_map.get(&notk).unwrap_or(&0); 
            res_map.insert(k.to_string(), (pos as u64, neg as u64));
        }
    }

    let out = File::create(Path::new("./dict.csv"))?;
    let mut writer = BufWriter::new(out);
    for k in res_map.keys() {
        let (pos, neg) = res_map[k];
        if pos + neg >= 30 {
            writer.write_all(format!("{}; {}; {}; {}\n", k, pos, neg, pos + neg).as_bytes())?;
        }
    }

    Ok(())
}
