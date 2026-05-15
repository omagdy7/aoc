use std::vec;

struct Identifiers {
    ids: Vec<(usize, usize)>,
}

impl From<&str> for Identifiers {
    fn from(value: &str) -> Self {
        let id_ranges: Vec<&str> = value.split(',').collect();
        let ids: Vec<_> = id_ranges
            .iter()
            .map(|id| id.split_once('-').unwrap())
            .map(|(first, last)| {
                (
                    first.trim().parse::<usize>().unwrap(),
                    last.trim().parse::<usize>().unwrap(),
                )
            })
            .collect();
        Self { ids }
    }
}

fn is_invalid_id(id: usize) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();
    let (first_half, second_half) = id_str.split_at(id_str_len / 2);
    first_half == second_half
}

fn is_invalid_id_more_than_twice(id: usize) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();

    (1..id_str_len).any(|i| {
        id_str
            .as_bytes()
            .chunks(i)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect::<Vec<&str>>()
            .windows(2)
            .all(|w| w[0] == w[1])
    })
}

fn solve_part_one(data: &str) -> usize {
    Identifiers::from(data)
        .ids
        .iter()
        .map(|&(start, last)| {
            (start..=last)
                .filter(|&id| is_invalid_id(id))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn solve_part_two(data: &str) -> usize {
    Identifiers::from(data)
        .ids
        .iter()
        .map(|&(start, last)| {
            (start..=last)
                .filter(|&id| is_invalid_id_more_than_twice(id))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let test = include_str!("../input/day_2.test");
    let prod = include_str!("../input/day_2.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}
