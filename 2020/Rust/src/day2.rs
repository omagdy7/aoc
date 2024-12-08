struct Policy {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl From<&str> for Policy {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(':').unwrap();
        let (range, letter) = left.split_once(' ').unwrap();
        let (min, max) = range.split_once("-").unwrap();
        let password = right.trim_start().to_string();

        Policy {
            min: min.parse::<u32>().unwrap(),
            max: max.parse::<u32>().unwrap(),
            letter: letter.chars().next().unwrap(),
            password,
        }
    }
}

fn check_policy(policy: &Policy) -> bool {
    let count = policy.password.matches(policy.letter).count() as u32;
    count >= policy.min && count <= policy.max
}

fn check_policy_part_two(policy: &Policy) -> bool {
    let mut count = 0;
    let policy_password = policy.password.as_bytes();
    if policy_password[(policy.min - 1) as usize] == policy.letter as u8 {
        count += 1;
    }
    if policy_password[(policy.max - 1) as usize] == policy.letter as u8 {
        count += 1;
    }
    count == 1
}

fn solve_part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| Policy::from(line))
        .filter(|policy| check_policy(policy))
        .count()
}

fn solve_part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| Policy::from(line))
        .filter(|policy| check_policy_part_two(policy))
        .count()
}

fn main() {
    let test = include_str!("../input/day_2.test");
    let prod = include_str!("../input/day_2.prod");
    println!("Test_1: {}", solve_part_one(test));
    println!("Prod_1: {}", solve_part_one(prod));
    println!("Test_2: {}", solve_part_two(test));
    println!("Prod_2: {}", solve_part_two(prod));
}
