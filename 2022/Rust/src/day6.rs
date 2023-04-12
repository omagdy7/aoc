use std::collections::HashSet;

fn solve_functional(data: &str, window: usize) -> usize {
    data.as_bytes()
        .windows(window)
        .map(|arr| {
            arr.into_iter()
                .map(|ele| *ele as usize)
                .collect::<HashSet<_>>()
        })
        .position(|set| set.len() == window)
        .unwrap()
        + window
}

fn solve_part_one(data: &str) -> usize {
    let v: Vec<char> = data.chars().collect();
    for i in 0..v.len() - 4 {
        let mut st: HashSet<char> = HashSet::new();
        for j in i..i + 4 {
            st.insert(v[j]);
        }
        if st.len() == 4 {
            return i + 4;
        }
    }
    unreachable!()
}

fn solve_part_two(data: &str) -> usize {
    let v: Vec<char> = data.chars().collect();
    for i in 0..v.len() - 14 {
        let mut st: HashSet<char> = HashSet::new();
        for j in i..i + 14 {
            st.insert(v[j]);
        }
        if st.len() == 14 {
            return i + 14;
        }
    }
    unreachable!()
}

fn main() {
    let data_test = include_str!("../input/day6.test");
    let data_prod = include_str!("../input/day6.prod");

    println!("{}", solve_part_one(data_test));
    println!("{}", solve_part_one(data_prod));
    println!("{}", solve_part_two(data_test));
    println!("{}", solve_part_two(data_prod));

    println!("{}", solve_functional(data_test, 4));
    println!("{}", solve_functional(data_prod, 4));
    println!("{}", solve_functional(data_test, 14));
    println!("{}", solve_functional(data_prod, 14));
}
