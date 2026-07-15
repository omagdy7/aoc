#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities_to_fit: [isize; 6],
}

#[derive(Debug)]
struct DataInput {
    presents: Vec<[char; 9]>,
    regions: Vec<Region>,
}

impl From<&str> for DataInput {
    fn from(value: &str) -> Self {
        let sections: Vec<&str> = value.split("\n\n").collect();
        let (presents, regions) = sections.split_at(6);
        let presents: Vec<[char; 9]> = presents
            .iter()
            .map(|present| {
                let present: [char; 9] = present
                    .chars()
                    .filter(|&ch| ch == '#' || ch == '.')
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("expected exactly 9 chars");
                present
            })
            .collect();
        let regions: Vec<Region> = regions[0]
            .lines()
            .map(|region| {
                let (area, qs) = region.split_once(':').unwrap();
                let (width, height) = area
                    .split_once('x')
                    .map(|(w, h)| (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap()))
                    .unwrap();
                let quantities: [isize; 6] = qs
                    .split(' ')
                    .filter_map(|ch| ch.parse::<isize>().ok())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("expected exactly 6 digits");
                Region {
                    width,
                    height,
                    quantities_to_fit: quantities,
                }
            })
            .collect();
        Self { presents, regions }
    }
}

fn solve_part_one(data: &str) -> i64 {
    let input = DataInput::from(data);
    let actual_part_of_shape_blocks: Vec<usize> = input
        .presents
        .iter()
        .map(|shape| shape.iter().filter(|block| **block == '#').count())
        .collect();
    let mut ans: i64 = 0;
    for region in input.regions.iter() {
        let area = region.width * region.height;
        let count: usize = region
            .quantities_to_fit
            .iter()
            .enumerate()
            .map(|(i, &q)| actual_part_of_shape_blocks[i] * q as usize)
            .sum();
        if count <= area {
            ans += 1;
        }
    }
    ans
}

fn main() {
    let test = include_str!("../input/day_12.test");
    let prod = include_str!("../input/day_12.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
}
