use std::collections::HashMap;
use std::time::Instant;

use aoc_2022::Day;
use aoc_2022::PartResult;

pub struct DayInstance;

const CONVERT_LOWERCASE_VALUE: i32 = 96;
const CONVERT_UPPERCASE_VALUE: i32 = 38;

impl Day<i32> for DayInstance {
    fn part1(&self, input: &Vec<String>) -> PartResult<i32> {
        let now = Instant::now();
        let mut sum: i32 = 0;

        for line in input {
            let first_half: &Vec<char> = &line[..line.len() / 2].chars().collect();
            let second_half: &Vec<char> = &line[line.len() / 2..].chars().collect();

            let mut map: HashMap<char, bool> = HashMap::new();

            for char in first_half {
                map.insert(*char, true);
            }
            for char in second_half {
                if let Some(_) = map.get(char) {
                    sum += *char as i32
                        - if char.is_lowercase() {
                            CONVERT_LOWERCASE_VALUE
                        } else {
                            CONVERT_UPPERCASE_VALUE
                        };
                    break;
                }
            }
        }
        PartResult {
            solution: sum,
            execution_time: now.elapsed(),
        }
    }
    fn part2(&self, input: &Vec<String>) -> PartResult<i32> {
        let now = Instant::now();
        let mut sum: i32 = 0;

        let mut count: i8 = 0;

        let mut groups: Vec<Vec<char>> = Vec::new();
        for line in input {
            groups.push(line.chars().collect());
            count = count.wrapping_add(1);

            if count == 3 {
                let mut map: HashMap<char, bool> = HashMap::new();
                let mut maptwo: HashMap<char, bool> = HashMap::new();

                for char in &groups[0] {
                    map.insert(*char, true);
                }
                for char in &groups[1] {
                    if let Some(_) = map.get(char) {
                        maptwo.insert(*char, true);
                    }
                }
                for char in &groups[2] {
                    if let Some(_) = maptwo.get(char) {
                        sum += *char as i32
                            - if char.is_lowercase() {
                                CONVERT_LOWERCASE_VALUE
                            } else {
                                CONVERT_UPPERCASE_VALUE
                            };
                        break;
                    }
                }

                groups.clear();
                count = 0;
            }
        }

        PartResult {
            solution: sum,
            execution_time: now.elapsed(),
        }
    }
}
