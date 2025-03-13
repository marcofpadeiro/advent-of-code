use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let hello_world = read_input();

    println!("{:?}", hello_world);
}
fn read_input() -> Vec<String> {
    let file = File::open(format!("../input.txt")).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
