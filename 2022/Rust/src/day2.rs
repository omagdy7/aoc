// A -> Rock
// B -> Paper
// C -> Scissors
// D -> Rock

#[derive(Debug)]
enum Hand {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock(1),
            "B" | "Y" => Hand::Paper(2),
            "C" | "Z" => Hand::Scissors(3),
            _ => panic!("Should've been a valid hand"),
        }
    }
}

fn solve_part_one(data: &str) -> u32 {
    let mut score = 0;
    for line in data.lines() {
        let (h1, h2) = line.split_once(" ").unwrap();
        match Hand::from(h1) {
            Hand::Rock(_) => match Hand::from(h2) {
                Hand::Rock(x) => {
                    score += x + 3;
                }
                Hand::Paper(x) => {
                    score += x + 6;
                }
                Hand::Scissors(x) => {
                    score += x;
                }
            },
            Hand::Paper(_) => match Hand::from(h2) {
                Hand::Rock(x) => {
                    score += x;
                }
                Hand::Paper(x) => {
                    score += x + 3;
                }
                Hand::Scissors(x) => {
                    score += x + 6;
                }
            },
            Hand::Scissors(_) => match Hand::from(h2) {
                Hand::Rock(x) => {
                    score += x + 6;
                }
                Hand::Paper(x) => {
                    score += x;
                }
                Hand::Scissors(x) => {
                    score += x + 3;
                }
            },
        }
    }
    score
}

fn solve_part_two(data: &str) -> u32 {
    let mut score = 0;
    for line in data.lines() {
        let (h1, h2) = line.split_once(" ").unwrap();
        match Hand::from(h2) {
            Hand::Rock(_) => match Hand::from(h1) {
                Hand::Rock(_) => {
                    score += 3;
                }
                Hand::Paper(_) => {
                    score += 1;
                }
                Hand::Scissors(_) => {
                    score += 2;
                }
            },
            Hand::Paper(_) => match Hand::from(h1) {
                Hand::Rock(x) => {
                    score += x + 3;
                }
                Hand::Paper(x) => {
                    score += x + 3;
                }
                Hand::Scissors(x) => {
                    score += x + 3;
                }
            },
            Hand::Scissors(_) => match Hand::from(h1) {
                Hand::Rock(_) => {
                    score += 2 + 6;
                }
                Hand::Paper(_) => {
                    score += 3 + 6;
                }
                Hand::Scissors(_) => {
                    score += 1 + 6;
                }
            },
        }
    }
    score
}

fn main() {
    let data_test = include_str!("../input/day2.test");
    let data_prod = include_str!("../input/day2.prod");
    println!("{}", solve_part_one(data_test));
    println!("{}", solve_part_one(data_prod));
    println!("{}", solve_part_two(data_test));
    println!("{}", solve_part_two(data_prod));
}
