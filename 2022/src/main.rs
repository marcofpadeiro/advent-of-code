use std::{env, process};

use aoc_2022::{read_input, Day, DayResult};

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
            part1: days::day1::DayInstance.part1(&input),
            part2: days::day1::DayInstance.part2(&input),
        },
        2 => DayResult {
            part1: days::day2::DayInstance.part1(&input),
            part2: days::day2::DayInstance.part2(&input),
        },
        3 => DayResult {
            part1: days::day3::DayInstance.part1(&input),
            part2: days::day3::DayInstance.part2(&input),
        },
        4 => DayResult {
            part1: days::day4::DayInstance.part1(&input),
            part2: days::day4::DayInstance.part2(&input),
        },
        _ => {
            println!("Needs to be 1 to 25!");
            process::exit(1);
        }
    };

    println!("{}", res)
}
