use std::collections::HashSet;

fn solve_part_one(data: &str) -> u32 {
    let mut almanac = Almanac::from(data);
    let mut seeds = almanac.seeds.clone();
    let mut nseeds = vec![];
    for i in (0..seeds.len()).step_by(2) {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            nseeds.push(j);
        }
    }
    almanac.seeds = nseeds;

    let mut seeds = almanac.seeds.clone();
    for (_, maps) in almanac.maps.iter() {
        for seed in seeds.iter_mut() {
            let tmp = seed.clone();
            for seed_map in maps.iter() {
                if seed_map.contains(tmp) {
                    *seed += (seed_map.dest as i64 - seed_map.src as i64) as i64;
                }
            }
        }
    }
    *seeds.iter().min().unwrap() as u32
}

fn solve_part_two(data: &str) -> u64 {
    let mut almanac = Almanac::from(data);
    let seeds = almanac.seeds.clone();
    let mut set: HashSet<i64> = HashSet::new();
    let mut sum = 0;
    for i in (0..seeds.len()).step_by(2) {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            set.insert(j);
        }
        sum += seeds[i] + seeds[i + 1];
    }
    dbg!(set.len());
    sum as u64
}

type Seeds = Vec<i64>;

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Seeds,
    maps: Vec<(&'a str, Vec<SeedMap>)>,
}

impl<'a> From<&'a str> for Almanac<'a> {
    fn from(data: &'a str) -> Self {
        let data: Vec<&str> = data.split("\n\n").collect();
        let (_, seeds) = data[0].split_once(": ").unwrap();
        let seeds: Vec<_> = seeds
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut maps: Vec<(&'a str, Vec<SeedMap>)> = Vec::new();
        for map in data.iter().skip(1) {
            let (desc, seed_maps) = map.split_once(':').unwrap();
            let (name, _) = desc.split_once(' ').unwrap();
            let mut v = vec![];
            for smap in seed_maps.trim_start().trim_end().split('\n') {
                v.push(SeedMap::from(smap));
            }
            maps.push((name, v));
        }
        Self { seeds, maps }
    }
}

#[derive(Debug)]
struct SeedMap {
    src: usize,
    dest: usize,
    range_len: usize,
}

impl SeedMap {
    fn contains(&self, seed: i64) -> bool {
        (self.src..self.src + self.range_len).contains(&(seed as usize))
    }
}

impl From<&str> for SeedMap {
    fn from(seed_map: &str) -> Self {
        let map: Vec<_> = seed_map
            .split(' ')
            .map(|x| x.parse::<usize>().expect("Should be parsabel to usize"))
            .collect();
        Self {
            src: map[1],
            dest: map[0],
            range_len: map[2],
        }
    }
}

fn main() {
    let test = include_str!("../input/day5.test");
    let prod = include_str!("../input/day5.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    // println!("part_2 test: {:?}", solve_part_two(test));
    // println!("part_2 prod {:?}", solve_part_two(prod));
}
