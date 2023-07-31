use std::time::Instant;

use aoc_2022::read_input;
use aoc_2022::{DayResult, PartResult};

fn main() {
    let input = read_input(1);

    println!(
        "{}",
        DayResult {
            part1: part1(&input),
            part2: part2(input)
        }
    );
}

fn part1(input: &Vec<String>) -> PartResult<u32> {
    let now = Instant::now();

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

    PartResult {
        solution: curr_max,
        execution_time: now.elapsed(),
    }
}

fn part2(input: Vec<String>) -> PartResult<u32> {
    let now = Instant::now();

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

    PartResult {
        solution: vals.into_iter().rev().take(3).sum::<u32>(),
        execution_time: now.elapsed(),
    }
}
