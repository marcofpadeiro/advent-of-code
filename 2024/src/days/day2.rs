use std::time::Instant;

use aoc_2024::{Day, PartResult};

pub struct Day2;

impl Day<String> for Day2 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut total = 0;

        for line in input {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if is_safe(&nums) {
                total += 1;
            }
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        let mut total = 0;

        for line in input {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if is_safe(&nums) {
                total += 1;
            } else {
                let mut is_valid = false;

                for i in 0..nums.len() {
                    let mut nums_copy = nums.clone();
                    nums_copy.remove(i);

                    if is_safe(&nums_copy) {
                        is_valid = true;
                        break;
                    }
                }

                if is_valid {
                    total += 1;
                }
            }
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let asc = nums[0] - nums[1] < 0;
    for x in nums.windows(2) {
        let res = validate(x[0], x[1], asc);
        if !res {
            return false;
        }
    }
    true
}

fn validate(num1: i32, num2: i32, expected_asc: bool) -> bool {
    let diff = num1 - num2;
    if expected_asc && diff > 0 || !expected_asc && diff < 0 {
        return false;
    }
    let diff = diff.abs();
    if diff < 1 || diff > 3 {
        return false;
    }
    return true;
}
