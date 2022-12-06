use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

const STACK_ELEMENT_WIDTH: usize = 3;
const STACKS_SIZE: usize = 9;

#[derive(Debug, Clone)]
struct Stack {
    stack: VecDeque<StackElement>,
}

impl FromStr for Stack {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stack {
            stack: s
                .lines()
                .map(|line| line.parse::<StackElement>().unwrap())
                .collect::<VecDeque<_>>(),
        })
    }
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack: VecDeque::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct StackElement {
    ele: char,
}

impl FromStr for StackElement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = s.chars().nth(1).unwrap();
        Ok(StackElement { ele: ch })
    }
}

#[derive(Debug)]
struct Command {
    num_of_times: usize,
    from: usize,
    target: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<usize> = s
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .filter(|c| c.bytes().all(|c| c.is_ascii_digit()))
            .map(|n| n.parse().unwrap())
            .collect();
        let (num_of_times, from, target) = (s[0], s[1], s[2]);
        Ok(Command {
            num_of_times,
            from,
            target,
        })
    }
}

fn apply_command(stacks: &mut Vec<Stack>, cmd: &Command, part1: bool) {
    if part1 {
        for _ in 0..cmd.num_of_times {
            let element_to_push = stacks[cmd.from - 1]
                .stack
                .pop_back()
                .expect("welp the stack is empty");
            stacks[cmd.target - 1].stack.push_back(element_to_push);
        }
    } else {
        let mut tmp_stack: VecDeque<StackElement> = VecDeque::new();
        for _ in 0..cmd.num_of_times {
            let element_to_push = stacks[cmd.from - 1]
                .stack
                .pop_back()
                .expect("welp the stack is empty");
            tmp_stack.push_back(element_to_push);
        }
        for _ in 0..cmd.num_of_times {
            stacks[cmd.target - 1]
                .stack
                .push_back(tmp_stack.pop_back().unwrap());
        }
    }
}

fn solve_part_one(data: &str) -> String {
    let mut stacks: Vec<Stack> = vec![Stack::new(); STACKS_SIZE];
    let stack = data.split("\n\n").nth(0).unwrap();
    for x in stack.lines() {
        let chars = x.chars().clone();
        for (i, ch) in x.chars().enumerate() {
            if ch == '[' {
                stacks[i / 4].stack.push_front(
                    chars.as_str()[i..i + STACK_ELEMENT_WIDTH]
                        .parse::<StackElement>()
                        .unwrap(),
                )
            }
        }
    }
    let commands: Vec<Command> = data
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|command| command.parse::<Command>().unwrap())
        .collect();

    for cmd in &commands {
        apply_command(&mut stacks, cmd, true);
    }

    let mut ans = "".to_string();
    for i in 0..STACKS_SIZE {
        if stacks[i].stack.len() >= 1 {
            ans += stacks[i]
                .stack
                .iter()
                .last()
                .unwrap()
                .ele
                .to_string()
                .as_str();
        }
    }
    ans
}

fn solve_part_two(data: &str) -> String {
    let mut stacks: Vec<Stack> = vec![Stack::new(); STACKS_SIZE];
    let stack = data.split("\n\n").nth(0).unwrap();
    for x in stack.lines() {
        let chars = x.chars().clone();
        for (i, ch) in x.chars().enumerate() {
            if ch == '[' {
                stacks[i / 4].stack.push_front(
                    chars.as_str()[i..i + STACK_ELEMENT_WIDTH]
                        .parse::<StackElement>()
                        .unwrap(),
                )
            }
        }
    }
    let commands: Vec<Command> = data
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|command| command.parse::<Command>().unwrap())
        .collect();

    for cmd in &commands {
        apply_command(&mut stacks, cmd, false);
    }

    let mut ans = "".to_string();
    for i in 0..STACKS_SIZE {
        if stacks[i].stack.len() >= 1 {
            ans += stacks[i]
                .stack
                .iter()
                .last()
                .unwrap()
                .ele
                .to_string()
                .as_str();
        }
    }
    ans
}

fn main() {
    let data_test = include_str!("../data/day5.test");
    let data_prod = include_str!("../data/day5.prod");

    println!("part one test: {}", solve_part_one(data_test));
    println!("part one prod: {}", solve_part_one(data_prod));
    println!("part two test: {}", solve_part_two(data_test));
    println!("part two prod: {}", solve_part_two(data_prod));
}
