use std::collections::HashMap;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_vector(numbers: &[u64]) -> Option<u64> {
    if numbers.is_empty() {
        return None;
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    Some(result)
}

#[derive(Debug)]
struct Tree<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Tree<'a> {
    fn get_children(&self, node: &'a str) -> Option<(&'a str, &'a str)> {
        self.nodes.get(node).cloned()
    }
}

impl<'a> From<&'a str> for Tree<'a> {
    fn from(tree: &'a str) -> Self {
        let mut nodes = HashMap::new();

        for line in tree.lines() {
            let (node, rest) = line.split_once(" = ").unwrap();
            let val = node;
            let (left, right) = rest.trim_matches(&['(', ')']).split_once(", ").unwrap();
            nodes.insert(val, (left, right));
        }

        Tree { nodes }
    }
}

fn solve_part_one(data: &str) -> u64 {
    let (instructions, graph) = data.split_once("\n\n").unwrap();
    let tree = Tree::from(graph);
    let mut cur_state = "AAA";
    let mut ans = 0;
    while cur_state != "ZZZ" {
        for inst in instructions.chars() {
            match inst {
                'L' => {
                    cur_state = tree.get_children(cur_state).unwrap().0;
                }
                'R' => {
                    cur_state = tree.get_children(cur_state).unwrap().1;
                }
                _ => unreachable!(),
            }
            ans += 1;
        }
    }
    ans as u64
}

fn solve_part_two(data: &str) -> u64 {
    let (instructions, graph) = data.split_once("\n\n").unwrap();
    let tree = Tree::from(graph);
    let cur_state = Vec::from_iter(
        tree.nodes
            .iter()
            .filter_map(|(&p, _)| p.ends_with('A').then_some(p)),
    );

    let mut first_zs = vec![];

    for &state in cur_state.iter() {
        let mut cur = state;
        let mut i = 0;
        'simu: loop {
            for inst in instructions.chars() {
                match inst {
                    'L' => {
                        let next_state = tree.get_children(cur).unwrap().0;
                        if cur.ends_with('Z') {
                            break 'simu;
                        }
                        cur = next_state;
                    }
                    'R' => {
                        let next_state = tree.get_children(cur).unwrap().1;
                        if cur.ends_with('Z') {
                            break 'simu;
                        }
                        cur = next_state;
                    }
                    _ => unreachable!(),
                }
                i += 1;
            }
        }
        first_zs.push(i);
    }
    lcm_vector(&first_zs).unwrap()
}

fn main() {
    let test_1 = include_str!("../input/day8_1.test");
    let test_2 = include_str!("../input/day8_2.test");
    let prod = include_str!("../input/day8.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_2));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
