use core::range::RangeInclusive;
use std::collections::{HashMap, HashSet};

struct Database {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients_id: Vec<usize>,
}

impl From<&str> for Database {
    fn from(value: &str) -> Self {
        let (ranges, ingredient_ids) = value.split_once("\n\n").unwrap();
        let ranges: Vec<RangeInclusive<usize>> = ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                RangeInclusive::from(
                    start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap(),
                )
            })
            .collect();
        let ingredients_id = ingredient_ids
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();
        Self {
            ranges,
            ingredients_id,
        }
    }
}

fn solve_part_one(data: &str) -> usize {
    let database = Database::from(data);
    database
        .ingredients_id
        .iter()
        .filter(|id| database.ranges.iter().any(|range| range.contains(&id)))
        .count()
}

fn merge_ranges(ranges: &mut Vec<RangeInclusive<usize>>) {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut new_ranges: Vec<RangeInclusive<usize>> = vec![];
    let n = ranges.len();
    let mut excludes_idxs = vec![];
    for i in 0..n {
        let mut flag = false;
        for j in i + 1..n {
            if ranges[j].start <= ranges[i].last {
                flag = true;
                new_ranges.push(RangeInclusive::from(
                    ranges[i].start as usize..=ranges[j].last.max(ranges[i].last),
                ));
                excludes_idxs.push(j);
                break;
            }
        }
        if !flag && !excludes_idxs.contains(&i) {
            new_ranges.push(ranges[i]);
        }
    }
    new_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    *ranges = new_ranges.clone();
}

fn solve_part_two(data: &str) -> usize {
    let database = Database::from(data);
    let mut ranges = database.ranges;
    loop {
        let len_before = ranges.len();
        merge_ranges(&mut ranges);
        if len_before == ranges.len() {
            break;
        }
    }
    ranges
        .iter()
        .map(|range| range.last - range.start + 1)
        .sum()
}

fn main() {
    let test = include_str!("../input/day_5.test");
    let prod = include_str!("../input/day_5.prod");
    println!("part_1 test: {}", solve_part_one(test));
    println!("part_1 prod: {}", solve_part_one(prod));
    println!("part_2 test: {}", solve_part_two(test));
    println!("part_2 prod: {}", solve_part_two(prod));
}
