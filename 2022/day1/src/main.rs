use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let input = file_to_vec(INPUT_PATH);

    part1(&input);
    part2(input);
}

fn part1(input: &Vec<String>) {
    let mut curr_max: u32 = 0;
    let mut curr_val: u32 = 0;

    for line in input {
        if line.is_empty() {
            if curr_val > curr_max {
                curr_max = curr_val;
            }
            curr_val = 0;
            continue;
        }

        curr_val += line.parse::<u32>().expect("Failed to parse to integer");
    }

    println!("part1: {}", curr_max);
}

fn part2(input: Vec<String>) {
    let mut vals: Vec<u32> = Vec::new();
    let mut curr_val: u32 = 0;

    for line in input {
        if line.is_empty() {
            vals.push(curr_val);
            curr_val = 0;
            continue;
        }
        curr_val += line.parse::<u32>().expect("Failed to parse to integer")
    }

    vals.sort();
    println!("part2: {}", vals.into_iter().rev().take(3).sum::<u32>());
}

fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
