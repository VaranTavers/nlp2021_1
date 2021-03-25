use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;

use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn parse_dict(filename: &str) -> std::io::Result<HashMap<String, f32>> {
    let mut dict = HashMap::new();


    let file = File::open(filename)?;
    let rd = BufReader::new(file);

    for line_op in rd.lines() {
        let line = line_op?;
        let sp = line.split(';').collect::<Vec<&str>>();
        dict.insert(sp[0].to_string(), parse_input!(sp[1], f32));
    }

    Ok(dict)
}