fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|x| x.chars().filter(|ch| ch.is_digit(10)).collect::<String>())
        .collect::<Vec<_>>()
        .iter()
        .map(|x| {
            format!("{}{}", x.chars().next().unwrap(), x.chars().last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn solve_part_two(data: &str) -> u32 {
    let nums = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut sum = 0;
    for line in data.lines() {
        let mut v = vec![];
        for (lit, dig) in nums.iter() {
            if let Some(idx) = line.find(lit) {
                v.push((idx, dig));
            }
            if let Some(idx) = line.find(dig) {
                v.push((idx, dig));
            }
            let line_rev = line.chars().rev().collect::<String>();
            let lit_rev = lit.chars().rev().collect::<String>();
            if let Some(idx) = line_rev.find(lit_rev.as_str()) {
                v.push((line_rev.len() - idx - lit_rev.len(), dig));
            }
            if let Some(idx) = line_rev.find(dig) {
                v.push((line_rev.len() - idx - 1, dig));
            }
        }
        v.sort();
        sum += format!("{}{}", v.first().unwrap().1, v.last().unwrap().1)
            .parse::<u32>()
            .unwrap();
    }
    sum
}

fn main() {
    let test_1 = include_str!("../input/day1_1.test");
    let test_2 = include_str!("../input/day1_2.test");
    let prod = include_str!("../input/day1.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_2));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
