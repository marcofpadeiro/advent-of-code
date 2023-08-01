use std::time::Instant;

use aoc_2022::Day;
use aoc_2022::PartResult;

pub struct DayInstance;

#[derive(Debug)]
struct Task {
    start: i32,
    end: i32,
}

impl Task {
    fn new(start: i32, end: i32) -> Self {
        Task { start, end }
    }
}

impl Day<i32> for DayInstance {
    fn part1(&self, input: &Vec<String>) -> PartResult<i32> {
        let now = Instant::now();
        let mut counter: i32 = 0;

        for line in input {
            let tasks: Vec<Task> = line
                .split(',')
                .map(|task_str| {
                    let nums: Vec<i32> = task_str
                        .split('-')
                        .map(|num_str| num_str.parse::<i32>().unwrap())
                        .collect();
                    Task::new(nums[0], nums[1])
                })
                .collect();

            if tasks[0].start <= tasks[1].start && tasks[0].end >= tasks[1].end
                || tasks[1].start <= tasks[0].start && tasks[1].end >= tasks[0].end
            {
                counter = counter.wrapping_add(1);
            }
        }
        PartResult {
            solution: counter,
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<i32> {
        let now = Instant::now();
        let mut counter: i32 = 0;

        for line in input {
            let tasks: Vec<Task> = line
                .split(',')
                .map(|task_str| {
                    let nums: Vec<i32> = task_str
                        .split('-')
                        .map(|num_str| num_str.parse::<i32>().unwrap())
                        .collect();
                    Task::new(nums[0], nums[1])
                })
                .collect();

            let sectors: Vec<i32> = (tasks[0].start..=tasks[0].end).collect();
            for i in tasks[1].start..=tasks[1].end {
                if sectors.contains(&i) {
                    counter = counter.wrapping_add(1);
                    break;
                }
            }
        }
        PartResult {
            solution: counter,
            execution_time: now.elapsed(),
        }
    }
}
