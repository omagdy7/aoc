use std::collections::HashSet;

fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|line| -> u32 {
            let (l, r) = line.split_at(line.len() / 2);
            let l = l.chars().collect::<HashSet<_>>();
            let r = r.chars().collect::<HashSet<_>>();
            let mut acc = 0;
            for x in l.intersection(&r) {
                if x.is_ascii_uppercase() {
                    acc += (*x as u8 - 'A' as u8 + 27) as u32;
                } else {
                    acc += (*x as u8 - 'a' as u8 + 1) as u32;
                };
            }
            acc
        })
        .sum()
}

fn solve_part_two(data: &str) -> u32 {
    let vecs: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .collect();

    vecs.chunks(3)
        .map(|x| -> u32 {
            let mut acc: u32 = 0;
            for i in x[0]
                .iter()
                .filter(|ch| x[1].contains(ch))
                .filter(|ch| x[2].contains(ch))
            {
                if i.is_ascii_uppercase() {
                    acc += (*i as u8 - 'A' as u8 + 27) as u32;
                } else {
                    acc += (*i as u8 - 'a' as u8 + 1) as u32;
                };
            }
            acc
        })
        .sum()
}

fn main() {
    let data = include_str!("../input/day3.prod");
    println!("{}", solve_part_one(data));
    println!("{}", solve_part_two(data));
}
