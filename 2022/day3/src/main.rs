use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

const CONVERT_LOWERCASE_VALUE: i32 = 96;
const CONVERT_UPPERCASE_VALUE: i32 = 38;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let input = file_to_vec(INPUT_PATH);

    part1(&input);
    part2(input);
}

fn part1(input: &Vec<String>) {
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
    println!("part1: {}", sum);
}

fn part2(input: Vec<String>) {
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
    println!("part2: {}", sum);
}

fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
