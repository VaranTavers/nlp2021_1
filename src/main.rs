use std::path::Path;

use std::collections::HashMap;

mod dict;
mod model;
mod parser;

use crate::model::Settings;

use crate::dict::get_sentiment_dict;
use crate::dict::write_sent_dict;

use crate::parser::parse_dict;
use crate::parser::parse_file;
use crate::parser::test_file;

fn test(folder_name: &str, signum: i32, dict: &HashMap<String, f32>, settings: &Settings) -> std::io::Result<(i32, i32)> {
    let folder = std::fs::read_dir(folder_name)?;
    let mut num = 0;
    let mut correct = 0;
    for entry in folder {
        num += 1;
        let mut ret = HashMap::new();
        parse_file(&(entry.unwrap().path()), &mut ret, &settings)?;
        let sent = ret.keys().fold(0.0, |acc: f32, w: &String| {
            acc + dict.get(w).unwrap_or(&0.0)
        });
        if sent * signum as f32 > 0.0 {
            correct += 1;
        }
    }

    Ok((num as i32, correct))
}

fn main() -> std::io::Result<()> {
    let settings = Settings::default();

    if !(Path::new("./dict.csv").exists()) {
        println!("The dictionary file is missing. Rebuilding...");
        let dict = get_sentiment_dict(&settings)?;
        write_sent_dict("./dict.csv", dict)?;
        println!("The dictionary file has been rebuilt!");
    }

    let dict = parse_dict("./dict.csv")?;
    
    let (pos_n, pos_c) = test("./pos/", 1, &dict, &settings)?;
    let (neg_n, neg_c) = test("./neg/", 1, &dict, &settings)?;

    println!("Positive: {} / {} = {}%", pos_c, pos_n, pos_c as f32 / pos_n as f32 * 100.0);
    println!("Negative: {} / {} = {}%", neg_c, neg_n, neg_c as f32 / neg_n as f32 * 100.0);

    let (yelp_n, yelp_c) = test_file(Path::new("./test_data/amazon_cells_labelled.txt"), &dict, &settings)?;

    println!("Yelp: {} / {} = {}%", yelp_c, yelp_n, yelp_c as f32 / yelp_n as f32 * 100.0);

    Ok(())
}
