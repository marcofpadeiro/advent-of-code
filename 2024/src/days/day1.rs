use std::{collections::HashMap, time::Instant};

use aoc_2024::{Day, PartResult};

pub struct Day1;

impl Day<String> for Day1 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        let mut total = 0;

        let mut left_list: Vec<u32> = vec![];
        let mut right_list: Vec<u32> = vec![];

        for line in input {
            let line: Vec<&str> = line.split(' ').collect();
            left_list.push(line[0].parse::<u32>().unwrap());
            right_list.push(line[3].parse::<u32>().unwrap());
        }

        left_list.sort();
        right_list.sort();

        left_list.iter().enumerate().for_each(|(i, x)| {
            total += diff(x, &right_list[i]);
        });

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        let mut total = 0;

        let mut list: HashMap<u32, (u32, bool)> = HashMap::new();

        for line in input {
            let line: Vec<&str> = line.split(' ').collect();
            let left = line[0].parse::<u32>().unwrap();
            let right = line[3].parse::<u32>().unwrap();

            let entry = list.entry(left).or_insert((0, true));
            if !entry.1 {
                entry.1 = true;
                total += left * entry.0;
            }

            let counter = list.entry(right).or_insert((0, false));
            if counter.1 {
                total += right;
            } else {
                counter.0 += 1;
            }
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
}
fn diff(value1: &u32, value2: &u32) -> u32 {
    if value1 > value2 {
        return value1 - value2;
    } else if value2 > value1 {
        return value2 - value1;
    }
    return 0;
}
