use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT_PATH: &str = "input.txt";

struct Round {
    my_play: char,
    their_play: char,
}

impl Round {
    pub fn determine_round_points_part1(&self, points_system: &HashMap<char, i32>) -> i32 {
        let mut points = 0;

        // get play points
        points += points_system.get(&self.my_play).unwrap();

        let my_play = points_system.get(&self.my_play).unwrap();
        let their_play = points_system.get(&self.their_play).unwrap();

        let diff = (my_play - their_play + 3) % 3;
        points
            + match diff {
                1 => 6,
                2 => 0,
                _ => 3,
            }
    }
    pub fn determine_round_points_part2(&self, points_system: &HashMap<char, i32>) -> i32 {
        match &self.my_play {
            'X' => match &self.their_play {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            },
            'Y' => points_system.get(&self.their_play).unwrap() + 3,
            'Z' => match &self.their_play {
                'A' => 2 + 6,
                'B' => 3 + 6,
                'C' => 1 + 6,
                _ => 0,
            },
            _ => 0,
        }
    }
}

fn main() {
    let input = file_to_vec(INPUT_PATH);

    let points_system = initialize_points_system();
    let mut total_points_part1 = 0;
    let mut total_points_part2 = 0;

    for line in input {
        let round: Round = Round {
            their_play: line.chars().nth(0).unwrap(),
            my_play: line.chars().nth(2).unwrap(),
        };
        total_points_part1 += round.determine_round_points_part1(&points_system);
        total_points_part2 += round.determine_round_points_part2(&points_system);
    }

    println!("part1: {}", total_points_part1);
    print!("part2: {}", total_points_part2);
}

fn initialize_points_system() -> HashMap<char, i32> {
    let mut points_system: HashMap<char, i32> = HashMap::new();

    points_system.insert('A', 1); // rock
    points_system.insert('B', 2); // paper
    points_system.insert('C', 3); // scissors
    points_system.insert('X', 1); // rock
    points_system.insert('Y', 2); // paper
    points_system.insert('Z', 3); // scissors

    points_system
}

fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
