use std::{env, process};

use aoc_2024::{read_input, Day, DayResult};
use days::*;

mod days;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let day: i8 = args
        .get(0)
        .expect("Choose a day from 1 to 25")
        .parse()
        .expect("Choose a day from 1 to 25");

    let input = read_input(day);
    let res: DayResult<_> = match day {
        1 => DayResult {
            part1: day1::Day1.part1(&input),
            part2: day1::Day1.part2(&input),
        },
        2 => DayResult {
            part1: day2::Day2.part1(&input),
            part2: day2::Day2.part2(&input),
        },
        3 => DayResult {
            part1: day3::Day3.part1(&input),
            part2: day3::Day3.part2(&input),
        },
        4 => DayResult {
            part1: day4::Day4.part1(&input),
            part2: day4::Day4.part2(&input),
        },
        5 => DayResult {
            part1: day5::Day5.part1(&input),
            part2: day5::Day5.part2(&input),
        },
        6 => DayResult {
            part1: day6::Day6.part1(&input),
            part2: day6::Day6.part2(&input),
        },
        _ => {
            println!("Needs to be 1 to 25!");
            process::exit(1);
        }
    };

    println!("{}", res)
}
