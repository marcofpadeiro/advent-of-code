use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT_PATH: &str = "input.txt";

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

fn main() {
    let input = file_to_vec(INPUT_PATH);

    part1(&input);
    part2(input);
}

fn part1(input: &Vec<String>) {
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
    println!("part1: {}", counter);
}

fn part2(input: Vec<String>) {
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
    println!("part2: {}", counter);
}
fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
