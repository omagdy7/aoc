type Reports = Vec<Vec<i32>>;

fn parse_input(data: &str) -> Reports {
    data.lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe_recursive(report: &[i32]) -> bool {
    match report {
        [] => false,
        [_] => false,
        [a, b] => (1..=3).contains(&(a - b).abs()),
        [a, b, rest @ ..] => (1..=3).contains(&(a - b).abs()) && is_safe_recursive(rest),
    }
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    for i in 0..report.len() - 1 {
        if increasing {
            let diff = report[i + 1] - report[i];
            if report[i] >= report[i + 1] || diff > 3 || diff < 1 {
                return false;
            }
        } else {
            let diff = report[i] - report[i + 1];
            if report[i] <= report[i + 1] || diff > 3 || diff < 1 {
                return false;
            }
        }
    }
    return true;
}

fn solve_part_one(data: &str) -> usize {
    let reports = parse_input(data);
    reports.iter().filter(|&report| is_safe(report)).count()
}

fn solve_part_two(data: &str) -> usize {
    let reports = parse_input(data);
    let mut cnt: usize = 0;
    reports
        .iter()
        .filter(|&report| !is_safe(report))
        .for_each(|report| {
            for i in 0..report.len() {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if is_safe(&report_copy) {
                    cnt += 1;
                    break;
                }
            }
        });
    cnt + reports.iter().filter(|&report| is_safe(report)).count()
}
fn main() {
    let test_1 = include_str!("../input/day_2.test");
    let prod = include_str!("../input/day_2.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_1));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
