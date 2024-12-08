type History = Vec<i64>;

#[derive(Debug)]
struct Report {
    histories: Vec<History>,
}

impl From<&str> for Report {
    fn from(report: &str) -> Self {
        Self {
            histories: report
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<History>()
                })
                .collect::<Vec<History>>(),
        }
    }
}

fn solve_part_one(data: &str) -> u64 {
    let report = Report::from(data);
    let mut ans = 0;
    for history in report.histories.iter() {
        let mut differences = vec![];
        differences.push(history.clone());
        while !differences.last().unwrap().iter().all(|&x| x == 0) {
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<_>>(),
            )
        }

        for i in (0..differences.len() - 2).rev() {
            let last_last = differences[i + 1].last().unwrap() + differences[i].last().unwrap();
            differences[i].push(last_last);
        }
        ans += differences.first().unwrap().last().unwrap();
    }
    ans as u64
}

fn solve_part_two(data: &str) -> u64 {
    let report = Report::from(data);
    let mut ans = 0;
    for history in report.histories.iter() {
        let mut differences = vec![];
        differences.push(history.clone());
        while !differences.last().unwrap().iter().all(|&x| x == 0) {
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<_>>(),
            )
        }

        for i in (0..differences.len() - 2).rev() {
            let first_first = differences[i].first().unwrap() - differences[i + 1].first().unwrap();
            differences[i].insert(0, first_first);
        }
        ans += differences.first().unwrap().first().unwrap();
    }
    ans as u64
}

fn main() {
    let test_1 = include_str!("../input/day9_1.test");
    let test_2 = include_str!("../input/day9_2.test");
    let prod = include_str!("../input/day9.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_2));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
