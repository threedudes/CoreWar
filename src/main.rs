mod types;
mod parser;
mod macros;
mod simulator;
use std::fs;

fn main() {
    let content = fs::read_to_string("warrior.rc").expect("Can't read file !");
    let warrior1 = parser::parse(&content);
    let mut core = simulator::Core::new(8000);
    core.load_warriors(vec![warrior1]);
}