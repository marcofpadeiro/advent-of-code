use std::time::Instant;

use aoc_2024::{Day, PartResult};

pub struct Day7;

impl Day<String> for Day7 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let total = parse_lines(input, false);

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let total = parse_lines(input, true);

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
}

fn parse_lines(input: &Vec<String>, part2: bool) -> u64 {
    let mut total = 0;
    input.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let res: u64 = parts[0].parse().unwrap();
        let nums: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_valid(nums, 0, res, part2) {
            total += res;
        }
    });
    total
}

fn is_valid(nums: Vec<u64>, current: u64, res: u64, part2: bool) -> bool {
    if nums.is_empty() {
        return current == res;
    }

    let temp = nums.get(0).unwrap();
    let mut op: Vec<u64> = vec![current + temp, current * temp];

    if part2 {
        op.push(concat(current, *temp));
    };

    for x in op {
        if x > res {
            continue;
        }

        if is_valid(nums[1..].to_vec(), x, res, part2) {
            return true;
        }
    }

    false
}

fn concat(a: u64, b: u64) -> u64 {
    a as u64 * 10u64.pow(if b > 0 { b.ilog10() + 1 } else { 1 }) + b as u64
}
