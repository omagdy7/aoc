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
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .into_iter()
                .filter(|t| (time - t) * t > distance)
                .map(|_| distance)
                .count()
        })
        .product::<usize>() as u64
}
fn solve_part_two(data: &str) -> u64 {
    let race: Race<Part2> = Race::from(data);
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .into_iter()
                .filter(|t| (time - t) * t > distance)
                .map(|_| distance)
                .count()
        })
        .product::<usize>() as u64
}

fn main() {
    let test = include_str!("../input/day6.test");
    let prod = include_str!("../input/day6.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
