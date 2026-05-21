fn solve(data: &str, num_size: usize) -> usize {
    let mut ans = 0;
    for bank in data.lines() {
        let mut joltage = String::new();
        let mut deletions_left = bank.len() - num_size;
        for num in bank.bytes().map(|b| (b - b'0') as u32) {
            while let Some(&last) = joltage.as_bytes().last() {
                if (last - b'0') as u32 >= num || deletions_left == 0 {
                    break;
                }
                joltage.pop();
                deletions_left -= 1;
            }
            joltage.push(char::from(b'0' + num as u8));
        }
        joltage.truncate(num_size);
        ans += joltage.parse::<usize>().unwrap();
    }
    ans
}

fn solve_part_one(data: &str) -> usize {
    solve(data, 2)
}

fn solve_part_two(data: &str) -> usize {
    solve(data, 12)
}

fn main() {
    let test = include_str!("../input/day_3.test");
    let prod = include_str!("../input/day_3.prod");
    println!("part_1 test: {}", solve_part_one(test));
    println!("part_1 prod: {}", solve_part_one(prod));
    println!("part_2 test: {}", solve_part_two(test));
    println!("part_2 prod: {}", solve_part_two(prod));
}
