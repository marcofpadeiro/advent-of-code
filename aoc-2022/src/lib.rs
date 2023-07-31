use std::{
    fmt::{Display, Error, Formatter},
    fs::File,
    io::{prelude::*, BufReader},
    time::Duration,
};

pub fn read_input(day: i8) -> Vec<String> {
    let file = File::open(format!("inputs/day{}.txt", day)).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub struct PartResult<T: Display> {
    pub solution: T,
    pub execution_time: Duration,
}

pub struct DayResult<T: Display> {
    pub part1: PartResult<T>,
    pub part2: PartResult<T>,
}

impl<T: Display> Display for PartResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "  Solution: {}\n  Execution Time: {:?}",
            self.solution, self.execution_time
        )
    }
}

impl<T: Display> Display for DayResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Part 1:\n{}\n\nPart 2:\n{}", self.part1, self.part2)
    }
}

pub trait Day<T: Display> {
    fn part1(&self, input: &str) -> PartResult<T>;
    fn part2(&self, input: &str) -> PartResult<T>;
}
