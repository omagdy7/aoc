use std::collections::HashSet;

fn solve_part_one(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let numbers: Vec<i32> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    for &number in &numbers {
        let complement = 2020 - number;
        if seen.contains(&complement) {
            return number * complement;
        }
        seen.insert(number);
    }
    panic!("No two numbers sum to 2020!");
}

fn solve_part_two(input: &str) -> i32 {
    let numbers: Vec<i32> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    for (i, &first) in numbers.iter().enumerate() {
        let mut seen = HashSet::new();
        let target_sum = 2020 - first;
        for &second in &numbers[i + 1..] {
            let complement = target_sum - second;
            if seen.contains(&complement) {
                return first * second * complement;
            }
            seen.insert(second);
        }
    }
    panic!("No three numbers sum to 2020!");
}

fn main() {
    let test = include_str!("../input/day_1.test");
    let prod = include_str!("../input/day_1.prod");
    println!("Test_1: {}", solve_part_one(test));
    println!("Prod_1: {}", solve_part_one(prod));
    println!("Test_2: {}", solve_part_two(test));
    println!("Prod_2: {}", solve_part_two(prod));
}
