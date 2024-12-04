use std::time::Instant;

use aoc_2024::{Day, PartResult};

pub struct Day4;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

const DIRECTIONS_P2: [(i32, i32); 2] = [(1, 1), (1, -1)];

impl Day<String> for Day4 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();

        let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        let mut total = 0;

        for (x, line) in input.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == 'X' {
                    for direction in DIRECTIONS {
                        if step(&input, x as i32, y as i32, direction).4 {
                            total += 1;
                        }
                    }
                }
            }
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        let mut total = 0;

        for (x, line) in input.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                let mut valid_sides = 0;
                if *c == 'A' {
                    for direction in DIRECTIONS_P2 {
                        let left = (x as i32 + direction.0, y as i32 + direction.1);
                        let right = (x as i32 + direction.0 * -1, y as i32 + direction.1 * -1);

                        if left.0 < 0
                            || left.1 < 0
                            || left.0 > (input.len() - 1) as i32
                            || left.1 > (input[0].len() - 1) as i32
                        {
                            break;
                        }
                        if right.0 < 0
                            || right.1 < 0
                            || right.0 > (input.len() - 1) as i32
                            || right.1 > (input[0].len() - 1) as i32
                        {
                            break;
                        }

                        let (a, b) = (
                            input[left.0 as usize][left.1 as usize],
                            input[right.0 as usize][right.1 as usize],
                        );
                        valid_sides += match (a, b) {
                            ('M', 'S') | ('S', 'M') => 1,
                            _ => 0,
                        };
                    }
                    total += valid_sides / 2;
                }
            }
        }

        PartResult {
            solution: total.to_string(),
            execution_time: now.elapsed(),
        }
    }
}

fn step<'a>(
    array: &'a Vec<Vec<char>>,
    mut x: i32,
    mut y: i32,
    direction: (i32, i32),
) -> (&'a Vec<Vec<char>>, i32, i32, (i32, i32), bool) {
    let curr_char = array[x as usize][y as usize];
    let expected_char = match curr_char {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("what the hell"),
    };
    x += direction.0;
    y += direction.1;

    if x < 0 || y < 0 || y > (array.len() - 1) as i32 || x > (array[0].len() - 1) as i32 {
        return (array, x, y, direction, false);
    }

    let curr_char = array[x as usize][y as usize];

    if curr_char == expected_char {
        if curr_char == 'S' {
            return (array, x, y, direction, true);
        }
        return step(array, x, y, direction);
    }

    return (array, x, y, direction, false);
}
