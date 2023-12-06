use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug)]
struct Race<State = Part1> {
    times: Vec<u64>,
    distances: Vec<u64>,
    state: std::marker::PhantomData<State>,
}

struct Part1;
struct Part2;

impl From<&str> for Race<Part1> {
    fn from(value: &str) -> Self {
        let (times, distances) = value.split_once("\n").unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        let times = times
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let distances = distances
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            times,
            distances,
            state: PhantomData::<Part1>,
        }
    }
}

impl From<&str> for Race<Part2> {
    fn from(value: &str) -> Self {
        let (times, distances) = value.split_once("\n").unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        let times = times
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let distances = distances
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        Self {
            times: vec![times],
            distances: vec![distances],
            state: PhantomData::<Part2>,
        }
    }
}

fn solve_part_one(data: &str) -> u64 {
    let race: Race<Part1> = Race::from(data);
    let mut cnt = 0;
    let mut ans = 1;
    for (i, _) in race.times.iter().enumerate() {
        for t in 0..=race.times[i] {
            if (race.times[i] - t) * t > race.distances[i] {
                cnt += 1;
            }
        }
        ans *= cnt;
        cnt = 0;
    }
    ans
}
fn solve_part_two(data: &str) -> u64 {
    let race: Race<Part2> = Race::from(data);
    let mut ans = 0;
    for t in 0..=race.times[0] {
        if (race.times[0] - t) * t > race.distances[0] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    let test = include_str!("../input/day6.test");
    let prod = include_str!("../input/day6.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
