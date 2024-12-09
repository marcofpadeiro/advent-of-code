use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc_2024::{Day, PartResult};

pub struct Day8;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Day<String> for Day8 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut result: HashSet<Pos> = HashSet::new();

        let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
        let width = input.len();
        let height = input[0].len();
        let antinodes: HashMap<char, Vec<Pos>> = get_antinodes(&input);

        for (_, positions) in antinodes.iter() {
            for (i, pos1) in positions.iter().enumerate() {
                for pos2 in positions.iter().skip(i + 1) {
                    let (antinode1, antinode2) = calculate_positions(pos1, pos2);

                    if validate_position(&antinode1, width, height) {
                        result.insert(antinode1.clone());
                    }

                    if validate_position(&antinode2, width, height) {
                        result.insert(antinode2.clone());
                    }
                }
            }
        }

        PartResult {
            solution: result.len().to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let mut result: HashSet<Pos> = HashSet::new();

        let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
        let width = input.len();
        let height = input[0].len();
        let antinodes: HashMap<char, Vec<Pos>> = get_antinodes(&input);

        for (_, positions) in antinodes.iter() {
            for (i, pos1) in positions.iter().enumerate() {
                for pos2 in positions.iter().skip(i + 1) {
                    let dx = pos2.x - pos1.x;
                    let dy = pos2.y - pos1.y;

                    let mut step = 1;

                    loop {
                        let antinode1 = Pos {
                            x: pos1.x + dx * step,
                            y: pos1.y + dy * step,
                        };

                        let antinode2 = Pos {
                            x: pos2.x - dx * step,
                            y: pos2.y - dy * step,
                        };

                        let valid1 = validate_position(&antinode1, width, height);
                        let valid2 = validate_position(&antinode2, width, height);

                        if valid1 {
                            result.insert(antinode1);
                        }

                        if valid2 {
                            result.insert(antinode2);
                        }

                        if !valid1 && !valid2 {
                            break;
                        }

                        step += 1;
                    }
                }
            }
        }
        for (_, positions) in &antinodes {
            for pos in positions {
                result.insert(*pos);
            }
        }

        PartResult {
            solution: result.len().to_string(),
            execution_time: now.elapsed(),
        }
    }
}

fn get_antinodes(input: &Vec<Vec<char>>) -> HashMap<char, Vec<Pos>> {
    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();

    input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, c)| {
            if *c != '.' {
                let entry = map.entry(*c).or_insert(vec![]);
                entry.push(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            }
        });
    });

    map
}

fn calculate_positions(pos1: &Pos, pos2: &Pos) -> (Pos, Pos) {
    let antinode1 = Pos {
        x: 2 * pos1.x as i32 - pos2.x as i32,
        y: 2 * pos1.y as i32 - pos2.y as i32,
    };
    let antinode2 = Pos {
        x: 2 * pos2.x as i32 - pos1.x as i32,
        y: 2 * pos2.y as i32 - pos1.y as i32,
    };

    (antinode1, antinode2)
}

fn validate_position(pos: &Pos, width: usize, height: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < width as i32 && pos.y < height as i32
}
