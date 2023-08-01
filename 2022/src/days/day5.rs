use std::time::Instant;

use aoc_2022::Day;
use aoc_2022::PartResult;

pub struct Day5;

enum State {
    Gathering,
    Stacking,
}

impl Day<String> for Day5 {
    fn part1(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
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

        PartResult {
            solution: outcome,
            execution_time: now.elapsed(),
        }
    }

    fn part2(&self, input: &Vec<String>) -> PartResult<String> {
        let now = Instant::now();
        let mut stack: [Vec<char>; 9] = Default::default();

        let mut state: State = State::Gathering;

        for line in input {
            match state {
                State::Gathering => state = gather_stack(&line, &mut stack),
                State::Stacking => rearrange_2(&line, &mut stack),
            }
        }

        let mut outcome = String::new();

        for pile in &stack {
            outcome += pile.last().unwrap().to_string().as_str();
        }

        PartResult {
            solution: outcome,
            execution_time: now.elapsed(),
        }
    }
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

fn rearrange_2(line: &String, stack: &mut [Vec<char>]) {
    if let Ok((amount, from_stack, to_stack)) = parse_rearrange(line) {
        let start = stack.get(from_stack).unwrap().len().wrapping_sub(amount);
        for _ in start..stack.get(from_stack).unwrap().len() {
            if let Some(val) = stack.get(from_stack).unwrap().get(start) {
                stack[to_stack].push(*val);
                stack.get_mut(from_stack).unwrap().remove(start);
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
