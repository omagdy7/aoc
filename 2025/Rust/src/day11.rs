use std::collections::HashMap;

type Node = String;
type Edges = Vec<Node>;
type Graph = HashMap<Node, Edges>;

fn dfs_2(
    node: &str,
    graph: &Graph,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), i64>,
) -> i64 {
    let seen_dac = seen_dac || node == "dac";
    let seen_fft = seen_fft || node == "fft";

    let key = (node.to_owned(), seen_dac, seen_fft);

    if let Some(&ans) = memo.get(&key) {
        return ans;
    }

    let ans = if let Some(neighbours) = graph.get(node) {
        neighbours
            .iter()
            .map(|neighbour| dfs_2(neighbour, graph, seen_dac, seen_fft, memo))
            .sum()
    } else if seen_dac && seen_fft {
        1
    } else {
        0
    };

    memo.insert(key, ans);
    ans
}

fn dfs(node: &str, graph: &Graph) -> i64 {
    if let Some(neighbours) = graph.get(node) {
        neighbours
            .iter()
            .map(|neighbour| dfs(neighbour, graph))
            .sum()
    } else {
        1
    }
}

fn build_graph(data: &str) -> Graph {
    let mut graph = Graph::new();
    for line in data.lines() {
        let (node, rest) = line.split_once(':').unwrap();
        let edges: Edges = rest.trim_start().split(' ').map(str::to_owned).collect();
        graph.insert(node.to_owned(), edges);
    }
    graph
}

fn solve_part_one(data: &str) -> i64 {
    let graph = build_graph(data);
    dfs("you", &graph)
}

fn solve_part_two(data: &str) -> i64 {
    let graph = build_graph(data);
    let mut memo = HashMap::new();
    dfs_2("svr", &graph, false, false, &mut memo)
}

fn main() {
    let test = include_str!("../input/day_11.test");
    let test_2 = include_str!("../input/day_11_2.test");
    let prod = include_str!("../input/day_11.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_2));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}
