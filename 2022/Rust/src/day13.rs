use std::{cmp::Ordering, fmt, str::FromStr};

#[derive(PartialEq, Eq, Clone)]
enum ListItem {
    List(Vec<ListItem>),
    Number(usize),
}

impl fmt::Debug for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListItem::List(arr) => f.debug_list().entries(arr.iter()).finish(),
            ListItem::Number(num) => write!(f, "{}", num),
        }
    }
}

impl ListItem {
    fn with_slice<T>(&self, f: impl FnOnce(&[ListItem]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ListItem::Number(a), ListItem::Number(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl FromStr for ListItem {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Remove whitespace from input
        let input = input.trim();

        // Check if input is empty
        if input.is_empty() {
            return Ok(ListItem::List(vec![]));
        }

        // Check if input is a number
        if let Ok(num) = input.parse::<usize>() {
            return Ok(ListItem::Number(num));
        }

        // Check if input is a list
        if input.starts_with('[') && input.ends_with(']') {
            // Remove brackets from input
            let input = &input[1..input.len() - 1];

            // Split input by commas
            let mut items = Vec::new();
            let mut start = 0;
            let mut level = 0;
            for (i, c) in input.char_indices() {
                match c {
                    '[' => level += 1,
                    ']' => level -= 1,
                    ',' if level == 0 => {
                        // Parse a list item
                        let item = ListItem::from_str(&input[start..i])?;
                        items.push(item);
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            // Parse the last list item
            let item = ListItem::from_str(&input[start..])?;
            items.push(item);

            return Ok(ListItem::List(items));
        }

        // Invalid input
        Err("Invalid input".to_string())
    }
}

type Packet = ListItem;

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

// [[1],[2,3,4]]
fn solve_part_one(data: &str) -> usize {
    let packets_pairs: Vec<&str> = data.split("\n\n").collect();
    let mut packets: Vec<PacketPair> = vec![];
    for pair in packets_pairs {
        let (first, last) = pair
            .split_once("\n")
            .expect("Should be splittable by newline");
        let first = ListItem::from_str(first).unwrap();
        let last = ListItem::from_str(last).unwrap();
        packets.push(PacketPair {
            left: first,
            right: last,
        });
    }
    let mut ans = 0;
    for (i, pair) in packets.iter().enumerate() {
        if pair.left < pair.right {
            ans += i + 1;
        }
    }
    ans
}

fn solve_part_two(data: &str) -> usize {
    let dividers = vec![
        Packet::List(vec![Packet::Number(2)]),
        Packet::List(vec![Packet::Number(6)]),
    ];

    let mut packets = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| Packet::from_str(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>()
}

fn main() {
    let test = include_str!("../input/day13.test");
    let prod = include_str!("../input/day13.prod");
    println!("Part1_test : {}", solve_part_one(test));
    println!("Part1_prod : {}", solve_part_one(prod));
    println!("{}", solve_part_two(test));
    println!("{}", solve_part_two(prod));
}
