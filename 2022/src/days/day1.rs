use std::time::Instant;

use aoc_2022::{Day, PartResult};

pub struct Day1;

impl Day<String> for Day1 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut curr_max: i32 = 0;
        let mut curr_val: i32 = 0;

        for line in input {
            if line.is_empty() {
                if curr_val > curr_max {
                    curr_max = curr_val;
                }
                curr_val = 0;
                continue;
            }

            curr_val += line.parse::<i32>().expect("Failed to parse to integer");
        }

        PartResult {
            solution: curr_max.to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut vals: Vec<i32> = Vec::new();
        let mut curr_val: i32 = 0;

        for line in input {
            if line.is_empty() {
                vals.push(curr_val);
                curr_val = 0;
                continue;
            }
            curr_val += line.parse::<i32>().expect("Failed to parse to integer")
        }

        vals.sort();

        PartResult {
            solution: vals.into_iter().rev().take(3).sum::<i32>().to_string(),
            execution_time: now.elapsed(),
        }
    }
}
