use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let nigga = read_input();

    println!("{:?}", nigga);
}
fn read_input() -> Vec<String> {
    let file = File::open(format!("../input.txt")).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
