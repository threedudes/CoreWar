mod types;
mod parser;
mod macros;
use std::fs;

fn main() {
    let content = fs::read_to_string("warrior.rc").expect("Can't read file !");
    let code = parser::parse(&content);
    println!("{:#?}", code);
}