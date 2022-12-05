fn solve_part_one(data: &str) -> u32 {
    data.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split("\n").flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .max()
        .unwrap()
}

fn solve_part_two(data: &str) -> u32 {
    let mut vec = data
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split("\n").flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect::<Vec<u32>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec.iter().take(3).sum()
}

fn main() {
    let data = include_str!("../data/day1.prod");
    println!("{:?}", solve_part_one(data));
    println!("{:?}", solve_part_two(data));
}
