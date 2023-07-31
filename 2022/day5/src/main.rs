use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT_PATH: &str = "input.txt";

enum State {
    Gathering,
    Stacking,
}

fn main() {
    let input = file_to_vec(INPUT_PATH);

    part1(&input);
}

fn part1(input: &Vec<String>) {
    let mut stack: [Vec<char>; 9] = Default::default();

    let mut state: State = State::Gathering;

    for line in input {
        match state {
            State::Gathering => state = gather_stack(line, &mut stack),
            State::Stacking => rearrange(line, &mut stack),
        }
    }

    let mut outcome = String::new();

    for pile in &stack {
        outcome += pile.last().unwrap().to_string().as_str();
    }

    println!("part1: {}", outcome);
}

fn gather_stack(line: &String, stack: &mut [Vec<char>]) -> State {
    let mut _count: usize = 0;

    let res: Vec<char> = line.chars().collect();
    for i in (1..line.len()).step_by(4) {
        if res[i].is_numeric() {
            stack.iter_mut().for_each(|vec| vec.reverse());
            return State::Stacking;
        }
        if res[i] != ' ' {
            stack[_count].push(res[i]);
        }
        _count = _count.wrapping_add(1);
    }
    State::Gathering
}

fn rearrange(line: &String, stack: &mut [Vec<char>]) {
    if let Ok((amount, from_stack, to_stack)) = parse_rearrange(line) {
        for _ in 0..amount {
            if let Some(val) = stack[from_stack].pop() {
                let to_stack = &mut stack[to_stack];
                to_stack.push(val);
            }
        }
    }
}

fn parse_rearrange(order: &String) -> Result<(usize, usize, usize), ()> {
    let mut numbers: Vec<i32> = Vec::new();

    if order.is_empty() {
        return Err(());
    }

    for word in order.split_whitespace() {
        if let Ok(number) = word.parse::<i32>() {
            numbers.push(number);
        }
    }

    Ok((
        numbers[0] as usize,
        (numbers[1] - 1) as usize,
        (numbers[2] - 1) as usize,
    ))
}

fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
