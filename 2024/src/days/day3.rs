use std::time::Instant;

use aoc_2024::{Day, PartResult};

pub struct Day3;

const PATTERN: &str = "mul(%,%)";

impl Day<String> for Day3 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let input = input.join("\n");

        PartResult {
            solution: chopped_solution(input).to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let input = input.join("");
        let mut total = 0;

        input.split("do()").for_each(|x| {
            let parts: Vec<&str> = x.split("don't()").collect();
            total += chopped_solution(parts[0].to_string());
        });

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
}

fn chopped_solution(input: String) -> u32 {
    let mut total = 0;
    let current_compare_char: Vec<char> = PATTERN.chars().collect();
    let mut first = 0;

    let mut count = 0;
    let mut number: String = String::new();
    let mut in_number: bool = false;

    input.chars().for_each(|c| {
        if current_compare_char[count] == '%' {
            in_number = true;
        }

        if in_number {
            if c.is_numeric() {
                number.push_str(c.to_string().as_str());
            } else if c == ',' {
                in_number = false;
                first = number.parse().unwrap();
                number = String::new();
            } else if c == ')' {
                in_number = false;
                count = 0;
                total += first * number.parse::<u32>().unwrap();
                first = 0;
            } else {
                first = 0;
                count = 0;
                number = String::new();
                in_number = false;
                if c == current_compare_char[count] {
                    count += 1;
                }
            }
        } else {
            number = String::new();
            in_number = false;
            if c == current_compare_char[count] {
                count += 1;
            } else {
                count = 0;
            }
        }
    });

    total
}
