#[allow(dead_code)]
use std::collections::HashMap;
use std::io::BufRead;
use std::time::Instant;
use std::{fs::File, io::BufReader};

#[allow(unused_variables)]
fn main() {
    let now = Instant::now();
    let input = read_input();
    let part2 = part2(input.get(0).unwrap());
    println!("{:?}", now.elapsed());
    let part1 = part1(input.get(0).unwrap());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &String) -> usize {
    let mut coord = (0, 0);
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();

    map.insert(coord, 1);
    input.chars().for_each(|c| {
        move_coord(c, &mut coord, &mut map);
        map.insert(coord, 1);
    });
    return map.len();
}

#[allow(unused_variables)]
fn part2(input: &String) -> usize {
    let mut coord = (0, 0);
    let mut coord_r = (0, 0);
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut normal = true;

    map.insert(coord, 1);
    input.chars().for_each(|c| {
        move_coord(c, if normal { &mut coord } else { &mut coord_r }, &mut map);
        normal = !normal;
    });

    return map.len();
}
fn move_coord(c: char, coord: &mut (i32, i32), map: &mut HashMap<(i32, i32), u32>) {
    match c {
        '<' => coord.0 -= 1,
        '>' => coord.0 += 1,
        '^' => coord.1 += 1,
        'v' => coord.1 -= 1,
        _ => panic!("should not be here"),
    };
    map.insert(*coord, 1);
}

fn read_input() -> Vec<String> {
    let file = File::open("../../inputs/day3.txt").expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
