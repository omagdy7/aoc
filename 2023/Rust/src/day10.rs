#[derive(Debug)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

#[derive(Debug)]
struct Maze {
    data: Vec<Vec<Pipe>>,
}

impl From<&str> for Maze {
    fn from(maze: &str) -> Self {
        use Pipe::*;
        let mut data: Vec<Vec<Pipe>> = vec![];
        for line in maze.lines() {
            let line = line.as_bytes();
            let mut pipe_line: Vec<Pipe> = vec![];
            for ch in line {
                match ch {
                    b'|' => pipe_line.push(Vertical),
                    b'-' => pipe_line.push(Horizontal),
                    b'L' => pipe_line.push(NorthEast),
                    b'J' => pipe_line.push(NorthWest),
                    b'7' => pipe_line.push(SouthWest),
                    b'F' => pipe_line.push(SouthEast),
                    b'.' => pipe_line.push(Ground),
                    b'S' => pipe_line.push(Start),
                    _ => {}
                }
            }
            data.push(pipe_line);
        }
        Maze { data }
    }
}

fn solve_part_one(data: &str) -> u64 {
    let maze = Maze::from(data);
    dbg!(maze);
    todo!()
}

fn solve_part_two(data: &str) -> u64 {
    todo!()
}

fn main() {
    let test_1 = include_str!("../input/day10_1.test");
    let test_2 = include_str!("../input/day10_2.test");
    let prod = include_str!("../input/day10.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    // println!("part_1 prod {:?}", solve_part_one(prod));
    // println!("part_2 test: {:?}", solve_part_two(test_2));
    // println!("part_2 prod {:?}", solve_part_two(prod));
}
