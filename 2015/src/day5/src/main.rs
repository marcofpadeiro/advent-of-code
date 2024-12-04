use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let input = read_input();
    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &Vec<String>) -> usize {
    let mut total = 0;
    input.iter().for_each(|line| {
        let mut past_char: char = ' ';
        let mut has_same = false;
        let mut vowel_counter = 0;
        let mut bad_word = false;

        for c in line.chars() {
            if c == past_char {
                has_same = true;
            }

            if is_vowel(c) {
                vowel_counter += 1;
            }

            let new = format!("{}{}", past_char, c);

            if new == "ab" || new == "cd" || new == "pq" || new == "xy" {
                bad_word = true;
                break;
            }

            past_char = c;
        }
        if has_same && vowel_counter >= 3 && !bad_word {
            total += 1;
        }
    });

    total
}
fn part2(input: &[String]) -> usize {
    0
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn read_input() -> Vec<String> {
    let file = File::open("../../inputs/day5.txt").expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
