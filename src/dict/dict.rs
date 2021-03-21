use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::collections::HashMap;

use crate::parser::parse_file;
use crate::model::Settings;


fn get_map_from_folder(folder_path: &Path, settings: &Settings) -> std::io::Result<HashMap<String, i32>> {
    let mut ret: HashMap<String, i32> = HashMap::new();
    
    let folder = std::fs::read_dir(folder_path)?;
    for entry in folder {
        parse_file(&(entry.unwrap().path()), &mut ret, settings)?;
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

pub fn get_sentiment_dict() -> std::io::Result<HashMap<String, f32>> {
    let settings = Settings::default();
    let neg_map = get_map_from_folder(Path::new("./neg"), &settings)?;
    println!("The negative reviews have been parsed!");
    let pos_map = get_map_from_folder(Path::new("./pos"), &settings)?;
    println!("The positive reviews have been parsed!");
    // Results in (positivity in [-1, 1], and predicted accuracy [0, 1])
    let mut res_map: HashMap<String, f32> = HashMap::new();

    for k in pos_map.keys() {
        let k = denot_word(k);
        let notk = format!("[NOT]{}", k);

        let pos = pos_map.get(k).unwrap_or(&0) + neg_map.get(&notk).unwrap_or(&0);
        let neg = neg_map.get(k).unwrap_or(&0) + pos_map.get(&notk).unwrap_or(&0); 
        
        let sum = pos + neg;
        if sum >= settings.min_occurence && sum <= settings.max_occurence {
            let pos_perc = pos as f32 / sum as f32;
            let neg_perc = neg as f32 / sum as f32;
            let res = pos_perc - neg_perc;
            res_map.insert(k.to_string(), res);
        }
    }

    for k in neg_map.keys() {
        let k = denot_word(k);
        if !res_map.contains_key(k) {
            let notk = format!("[NOT]{}", k);

            let pos = pos_map.get(k).unwrap_or(&0) + neg_map.get(&notk).unwrap_or(&0);
            let neg = neg_map.get(k).unwrap_or(&0) + pos_map.get(&notk).unwrap_or(&0); 
            let sum = pos + neg;
            if sum >= settings.min_occurence && sum <= settings.max_occurence {
                let pos_perc = pos as f32 / sum as f32;
                let neg_perc = neg as f32 / sum as f32;
                let res = pos_perc - neg_perc;
                res_map.insert(k.to_string(), res);
            }
        }
    }

    Ok(res_map)
}

pub fn write_sent_dict(filename: &str, res_map: HashMap<String, f32>) -> std::io::Result<()> {
    let out = File::create(Path::new(filename))?;
    let mut writer = BufWriter::new(out);
    for k in res_map.keys() {
        let val = res_map[k];
           writer.write_all(format!("{}; {}\n", k, val).as_bytes())?;
    }
    Ok(())
}