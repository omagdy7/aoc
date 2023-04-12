use std::{collections::HashSet, str::FromStr, string::ParseError};

#[derive(Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(" ").unwrap();
        let parsed_amount = amount
            .parse::<usize>()
            .expect("Should be parsable to usize");
        match dir {
            "R" => Ok(Command::Right(parsed_amount)),
            "L" => Ok(Command::Left(parsed_amount)),
            "U" => Ok(Command::Up(parsed_amount)),
            "D" => Ok(Command::Down(parsed_amount)),
            _ => unreachable!(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    NoOp,
}

impl Direction {
    fn new(diff: (i32, i32)) -> Self {
        use Direction::*;
        match diff {
            (2, 0) => Right,
            (-2, 0) => Left,
            (0, 2) => Up,
            (0, -2) => Down,
            (1, 2) => UpRight,
            (2, 1) => UpRight,
            (2, 2) => UpRight,
            (-2, 1) => UpLeft,
            (-1, 2) => UpLeft,
            (-2, 2) => UpLeft,
            (-1, -2) => DownLeft,
            (-2, -2) => DownLeft,
            (-2, -1) => DownLeft,
            (2, -1) => DownRight,
            (2, -2) => DownRight,
            (1, -2) => DownRight,
            _ => NoOp,
        }
    }
}

struct Simulation {
    unique_pos: HashSet<(i32, i32)>, // will mark tail positions
    head_pos: (i32, i32),
    tail_pos: Vec<(i32, i32)>,
}

impl Simulation {
    fn new(
        unique_pos: HashSet<(i32, i32)>,
        head_pos: (i32, i32),
        tail_pos: Vec<(i32, i32)>,
    ) -> Self {
        Simulation {
            unique_pos,
            head_pos,
            tail_pos,
        }
    }

    fn update_head_helper(&mut self, amount: &usize, tail_idx: usize, pos_increment: (i32, i32)) {
        for _ in 1..=*amount {
            self.head_pos.0 += pos_increment.0;
            self.head_pos.1 += pos_increment.1;
            for i in 0..9 {
                if !self.is_tail_one_block_away(i) {
                    self.update_tail(&self.get_direction(i), i);
                    self.unique_pos.insert(self.tail_pos[tail_idx]);
                }
            }
        }
    }

    fn update_head(&mut self, command: &Command, tail_idx: usize) {
        match command {
            Command::Up(amount) => {
                self.update_head_helper(amount, tail_idx, (0, 1));
            }
            Command::Down(amount) => {
                self.update_head_helper(amount, tail_idx, (0, -1));
            }
            Command::Right(amount) => {
                self.update_head_helper(amount, tail_idx, (1, 0));
            }
            Command::Left(amount) => {
                self.update_head_helper(amount, tail_idx, (-1, 0));
            }
        }
    }

    fn update_tail(&mut self, direction: &Direction, idx: usize) {
        use Direction::*;
        match direction {
            Up => self.tail_pos[idx].1 += 1,
            Down => self.tail_pos[idx].1 -= 1,
            Right => self.tail_pos[idx].0 += 1,
            Left => self.tail_pos[idx].0 -= 1,
            UpRight => {
                self.tail_pos[idx].1 += 1;
                self.tail_pos[idx].0 += 1
            }
            UpLeft => {
                self.tail_pos[idx].1 += 1;
                self.tail_pos[idx].0 -= 1
            }
            DownRight => {
                self.tail_pos[idx].1 -= 1;
                self.tail_pos[idx].0 += 1
            }
            DownLeft => {
                self.tail_pos[idx].1 -= 1;
                self.tail_pos[idx].0 -= 1
            }
            NoOp => {}
        }
    }

    fn get_pos_diff(&self, idx: usize) -> (i32, i32) {
        match idx {
            0 => {
                let dx = self.head_pos.0 - self.tail_pos[idx].0;
                let dy = self.head_pos.1 - self.tail_pos[idx].1;
                (dx, dy)
            }
            _ => {
                let dx = self.tail_pos[idx - 1].0 - self.tail_pos[idx].0;
                let dy = self.tail_pos[idx - 1].1 - self.tail_pos[idx].1;
                (dx, dy)
            }
        }
    }

    fn is_tail_one_block_away(&self, idx: usize) -> bool {
        match idx {
            0 => {
                let dx = i32::abs(self.head_pos.0 - self.tail_pos[idx].0);
                let dy = i32::abs(self.head_pos.1 - self.tail_pos[idx].1);
                dx + dy + 1 < 3
            }
            _ => {
                let dx = i32::abs(self.tail_pos[idx - 1].0 - self.tail_pos[idx].0);
                let dy = i32::abs(self.tail_pos[idx - 1].1 - self.tail_pos[idx].1);
                dx + dy + 1 < 3
            }
        }
    }

    fn get_direction(&self, idx: usize) -> Direction {
        Direction::new(self.get_pos_diff(idx))
    }
}

impl Command {
    fn apply(&self, simulation: &mut Simulation, tail_idx: usize) {
        simulation.update_head(self, tail_idx);
    }
}

fn solve_part_one(data: &str) -> usize {
    let tails = vec![(0, 0); 9];
    let mut simulation = Simulation::new(HashSet::new(), (0, 0), tails);
    let commands: Vec<_> = data
        .lines()
        .map(|x| Command::from_str(x).unwrap())
        .collect();
    for command in commands {
        command.apply(&mut simulation, 0)
    }
    simulation.unique_pos.len() + 1
}

fn solve_part_two(data: &str) -> usize {
    let tails = vec![(0, 0); 9];
    let mut simulation = Simulation::new(HashSet::new(), (0, 0), tails);
    let commands: Vec<_> = data
        .lines()
        .map(|x| Command::from_str(x).unwrap())
        .collect();
    for command in commands {
        command.apply(&mut simulation, 8)
    }
    simulation.unique_pos.len()
}

fn main() {
    let test = include_str!("../input/day9.test");
    let test_2 = include_str!("../input/day9_2.test");
    let prod = include_str!("../input/day9.prod");
    println!("part1 test: {}", solve_part_one(test));
    println!("part1 test: {}", solve_part_one(test_2));
    println!("part1 prod: {}", solve_part_one(prod));
    println!("part2 test: {}", solve_part_two(test));
    println!("part2 test: {}", solve_part_two(test_2));
    println!("part2 prod: {}", solve_part_two(prod));
}
