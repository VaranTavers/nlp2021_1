mod train_parser;
mod dict_parser;
mod test_parser;

pub use train_parser::parse_file;
pub use dict_parser::parse_dict;
pub use test_parser::test_file;