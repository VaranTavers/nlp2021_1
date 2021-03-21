use std::path::Path;

mod dict;
mod model;
mod parser;

use crate::dict::get_sentiment_dict;
use crate::dict::write_sent_dict;

fn main() -> std::io::Result<()> {
    if !(Path::new("./dict.csv").exists()) {
        println!("The dictionary file is missing. Rebuilding...");
        let dict = get_sentiment_dict()?;
        write_sent_dict("./dict.csv", dict)?;
        println!("The dictionary file has been rebuilt!");
    }



    Ok(())
}
