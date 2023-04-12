use std::{collections::VecDeque, str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(i64),
    Prod(i64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: VecDeque<i64>,
    inspected_items: i64,
    operation: Op,
    test: i64,
    monkey_true: i64,
    monkey_false: i64,
}

fn parse_items(line: &str) -> String {
    line.chars().skip_while(|&ch| ch != ':').skip(2).collect()
}

fn get_last_token(line: &str) -> i64 {
    line.split(" ")
        .last()
        .unwrap()
        .parse::<i64>()
        .expect("Should be parasble to i64")
}

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let items: VecDeque<i64> = parse_items(lines[1])
            .split(", ")
            .map(|x| x.parse::<i64>().expect("Should be parsable to i64"))
            .collect();
        let line_2: Vec<_> = lines[2].split(" ").collect();
        let v: Vec<_> = line_2.iter().rev().take(2).collect();
        let op = {
            match *v[0] {
                "old" => Op::Square,
                _ => match *v[1] {
                    "+" => Op::Add(v[0].parse::<i64>().unwrap()),
                    "*" => Op::Prod(v[0].parse::<i64>().unwrap()),
                    _ => unreachable!(),
                },
            }
        };
        let test = get_last_token(lines[3]);
        let monkey_true = get_last_token(lines[4]);
        let monkey_false = get_last_token(lines[5]);
        Ok(Monkey {
            starting_items: items,
            inspected_items: 0,
            operation: op,
            test,
            monkey_true,
            monkey_false,
        })
    }
}

fn solve_part_one(data: &str) -> i64 {
    let monkeys_input: Vec<_> = data.split("\n\n").collect();
    let mut monkeys: Vec<_> = monkeys_input
        .iter()
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].starting_items.is_empty() {
                use Op::*;
                monkeys[i].inspected_items += 1;
                let mut worry = monkeys[i].starting_items[0];
                match monkeys[i].operation {
                    Add(x) => worry += x,
                    Prod(x) => worry *= x,
                    Square => worry *= worry,
                };
                worry /= 3;
                let monkey_truth = monkeys[i].monkey_true as usize;
                let monkey_false = monkeys[i].monkey_false as usize;
                match worry % monkeys[i].test == 0 {
                    true => monkeys[monkey_truth].starting_items.push_back(worry),
                    false => monkeys[monkey_false].starting_items.push_back(worry),
                }
                monkeys[i].starting_items.pop_front();
            }
        }
    }
    let mut inspected_items: Vec<_> = monkeys.iter().map(|x| x.inspected_items).collect();
    inspected_items.sort();
    inspected_items.reverse();
    inspected_items.iter().take(2).product()
}

fn solve_part_two(data: &str) -> i64 {
    let monkeys_input: Vec<_> = data.split("\n\n").collect();
    let mut monkeys: Vec<_> = monkeys_input
        .iter()
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let divisors_product: i64 = monkeys.iter().map(|x| x.test).product();
            while !monkeys[i].starting_items.is_empty() {
                use Op::*;
                monkeys[i].inspected_items += 1;
                let mut worry = monkeys[i].starting_items[0];
                worry %= divisors_product;
                match monkeys[i].operation {
                    Add(x) => worry = worry + x,
                    Prod(x) => worry = worry * x,
                    Square => worry = worry * worry,
                };
                let monkey_truth = monkeys[i].monkey_true as usize;
                let monkey_false = monkeys[i].monkey_false as usize;
                match worry % monkeys[i].test == 0 {
                    true => monkeys[monkey_truth].starting_items.push_back(worry),
                    false => monkeys[monkey_false].starting_items.push_back(worry),
                }
                monkeys[i].starting_items.pop_front();
            }
        }
    }
    let mut inspected_items: Vec<_> = monkeys
        .iter()
        .map(|x| x.inspected_items)
        .collect::<Vec<_>>();
    inspected_items.sort();
    inspected_items.reverse();
    inspected_items.iter().take(2).product()
}

fn main() {
    let test = include_str!("../input/day11.test");
    let prod = include_str!("../input/day11.prod");
    println!("part1: test {:?}", solve_part_one(test));
    println!("part1: prod {:?}", solve_part_one(prod));
    println!("part2: test {:?}", solve_part_two(test));
    println!("part2: prod {:?}", solve_part_two(prod));
}
