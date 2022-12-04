use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Rng {
    start: usize,
    end: usize,
}

impl Rng {
    fn intersects(&self, rng: &Rng) -> bool {
        if self.start <= rng.end && rng.start <= self.end {
            return true;
        }
        false
    }

    fn contains(&self, rng: &Rng) -> bool {
        if self.start >= rng.start && self.end <= rng.end {
            true
        } else {
            false
        }
    }
}

impl FromStr for Rng {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        let start_fromstr = start.parse::<usize>()?;
        let end_fromstr = end.parse::<usize>()?;
        Ok(Rng {
            start: start_fromstr,
            end: end_fromstr,
        })
    }
}

fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|line| -> u32 {
            let l = line.split_once(',').unwrap();
            let r1 = Rng::from_str(l.0).unwrap();
            let r2 = Rng::from_str(l.1).unwrap();
            (r1.contains(&r2) || r2.contains(&r1)) as u32
        })
        .sum()
}

fn solve_part_two(data: &str) -> u32 {
    data.lines()
        .map(|line| -> (Rng, Rng) {
            let l = line.split_once(',').unwrap();
            (Rng::from_str(l.0).unwrap(), Rng::from_str(l.1).unwrap())
        })
        .map(|rngs| rngs.0.intersects(&rngs.1) as u32)
        .sum()
}

fn main() {
    let data_test = include_str!("../data/day4.test");
    let data_prod = include_str!("../data/day4.prod");
    println!("{}", solve_part_one(data_test));
    println!("{}", solve_part_two(data_test));
    println!("{}", solve_part_one(data_prod));
    println!("{}", solve_part_two(data_prod));
}
