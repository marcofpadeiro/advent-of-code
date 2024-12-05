use std::{collections::HashMap, time::Instant};

use aoc_2024::{Day, PartResult};

pub struct Day5;

impl Day<String> for Day5 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut parse = true;
        let mut prio: HashMap<u32, usize> = HashMap::new();
        let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut total = 0;

        input.iter().for_each(|x| {
            if x == "" {
                rules.iter().for_each(|(k, v)| {
                    prio.insert(k.clone(), v.len());
                });
                parse = false;
                return;
            }
            if parse {
                let parts: Vec<u32> = x.split('|').map(|d| d.parse().unwrap()).collect();
                rules
                    .entry(parts[0])
                    .or_insert(vec![parts[1]])
                    .push(parts[1]);
            } else {
                let mut parts: Vec<u32> = x.split(',').map(|d| d.parse().unwrap()).collect();
            }
        });
        for rule in rules {
            println!("{:?}", rule);
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        PartResult {
            solution: "".to_string(),
            execution_time: now.elapsed(),
        }
    }
}
