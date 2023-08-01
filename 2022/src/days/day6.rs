use std::time::Instant;

use aoc_2022::Day;
use aoc_2022::PartResult;

pub struct Day6;

const MARKER_LENGTH_PART1: usize = 4;
const MARKER_LENGTH_PART2: usize = 14;

impl Day<String> for Day6 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        return PartResult {
            solution: solution(input.get(0).unwrap().as_bytes(), MARKER_LENGTH_PART1).to_string(),
            execution_time: now.elapsed(),
        };
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        return PartResult {
            solution: solution(input.get(0).unwrap().as_bytes(), MARKER_LENGTH_PART2).to_string(),
            execution_time: now.elapsed(),
        };
    }
}

fn solution(input: &[u8], marker_length: usize) -> usize {
    let mut res = 0;

    for i in 0..input.len() {
        let mut vec = Vec::new();

        for j in i..i + marker_length {
            if !vec.contains(&input[j]) {
                vec.push(input[j]);
            }
        }

        if vec.len() == marker_length {
            res = i + marker_length;
            break;
        }
    }

    res
}
