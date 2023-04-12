use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
enum Command {
    Addx(i32),
    NoOp,
}

struct Crt {
    width: usize,
    height: usize,
    pixels: Vec<String>,
}

struct Sprite {
    location: i32,
}

impl Crt {
    fn new(width: usize, height: usize) -> Self {
        let row = ".".repeat(width);
        let pixels = vec![row; height];
        Crt {
            width,
            height,
            pixels,
        }
    }

    fn is_overlapping_with_sprite(&self, sprite: &Sprite, cycle: i32) -> Option<i32> {
        let x = cycle % 40;
        if ((x - 1)..=(x + 1)).contains(&sprite.location) {
            return Some(x);
        }
        None
    }
}

impl Sprite {
    fn new(intial_location: i32) -> Self {
        Sprite {
            location: intial_location,
        }
    }
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Command::NoOp),
            _ => {
                let (_, value) = s.split_once(" ").unwrap();
                Ok(Command::Addx(
                    value.parse::<i32>().expect("Should be parsable to usize"),
                ))
            }
        }
    }
}

fn signal_strength(cycle: &usize, x_val: &i32) -> i32 {
    *cycle as i32 * x_val
}

fn solve_part_one(data: &str) -> i32 {
    let important_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut cur_cycle = 0_usize;
    let mut x_val = 1_i32;
    let mut total_signal_strength = 0_i32;
    let commands: Vec<_> = data
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();
    for command in commands {
        use Command::*;
        match command {
            Addx(val) => {
                for _ in 0..2 {
                    cur_cycle += 1;
                    if important_cycles.contains(&cur_cycle) {
                        total_signal_strength += signal_strength(&cur_cycle, &x_val);
                    }
                }
                x_val += val;
            }
            NoOp => {
                cur_cycle += 1;
                if important_cycles.contains(&cur_cycle) {
                    total_signal_strength += signal_strength(&cur_cycle, &x_val);
                }
            }
        }
    }
    total_signal_strength
}

fn render_grid(grid: &Vec<String>) {
    for row in grid {
        for ch in row.chars() {
            match ch {
                '#' => print!("🥳"),
                // '#' => print!("😉"),
                '.' => print!("🌑"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

fn solve_part_two(data: &str) {
    let mut cur_cycle = 0_i32;
    let mut y_val = 0_i32;
    let mut crt = Crt::new(40, 6);
    let mut sprite = Sprite::new(1);
    let commands: Vec<_> = data
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();
    for command in commands {
        use Command::*;
        match command {
            Addx(val) => {
                for _ in 0..2 {
                    if cur_cycle % 40 == 0 {
                        if cur_cycle != 0 {
                            y_val += 1;
                        }
                    }
                    match crt.is_overlapping_with_sprite(&sprite, cur_cycle) {
                        Some(idx) => crt.pixels[y_val as usize]
                            .replace_range((idx as usize)..=idx as usize, "#"),
                        None => {}
                    }
                    cur_cycle += 1;
                }
                sprite.location += val;
            }
            NoOp => {
                if cur_cycle % 40 == 0 {
                    if cur_cycle != 0 {
                        y_val += 1;
                    }
                }
                match crt.is_overlapping_with_sprite(&sprite, cur_cycle) {
                    Some(idx) => {
                        crt.pixels[y_val as usize].replace_range((idx as usize)..=idx as usize, "#")
                    }
                    None => {}
                }
                cur_cycle += 1;
            }
        }
    }
    render_grid(&crt.pixels);
    println!();
}

fn main() {
    let test = include_str!("../input/day10.test");
    let prod = include_str!("../input/day10.prod");
    println!("part1: test {:?}", solve_part_one(test));
    println!("part1: prod {:?}", solve_part_one(prod));
    solve_part_two(test);
    solve_part_two(prod);
}
