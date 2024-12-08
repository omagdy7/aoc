use rayon::prelude::*;

fn solve_part_one(data: &str) -> u32 {
    use std::cmp::min;
    let almanac = Almanac::from(data);
    let seeds = almanac.seeds.clone();
    let mut ans = i64::MAX;
    for seed in seeds.iter() {
        let mut mn = seed.clone();
        for (_, maps) in almanac.maps.iter() {
            let tmp = mn.clone();
            for seed_map in maps.iter() {
                if seed_map.contains(tmp) {
                    mn = mn + seed_map.dest as i64 - seed_map.src as i64;
                }
            }
        }
        ans = min(ans, mn);
    }
    ans as u32
}

fn solve_part_two_slow(data: &str) -> u64 {
    use std::cmp::min;
    let almanac = Almanac::from(data);
    let seeds = Vec::from_iter(almanac.seeds.clone().into_iter());
    let mut pars = seeds
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, _)| (i, i64::MAX))
        .collect::<Vec<_>>();
    // let n = seeds.len();
    pars.par_iter_mut().for_each(|(i, ans)| {
        for seed in seeds[*i]..seeds[*i] + seeds[*i + 1] {
            let mut mn = seed.clone();
            for (_, maps) in almanac.maps.iter() {
                let tmp = mn.clone();
                for seed_map in maps.iter() {
                    if seed_map.contains(tmp) {
                        mn = mn + seed_map.dest as i64 - seed_map.src as i64;
                    }
                }
            }
            *ans = min(*ans, mn)
        }
    });
    *pars.iter().map(|(_, ans)| ans).min().unwrap() as u64
}

fn solve_part_two(data: &str) -> u64 {
    let almanac = Almanac::from(data);
    let seeds = Vec::from_iter(almanac.seeds.clone().into_iter());
    let ranges = seeds
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, _)| (seeds[i]..seeds[i] + seeds[i + 1]))
        .collect::<Vec<_>>();
    let mut ans = 0;
    'outer: loop {
        let mut mn = ans;
        for (_, maps) in almanac.maps.iter().rev() {
            let tmp = mn.clone();
            for seed_map in maps.iter().rev() {
                if seed_map.contains_dst(tmp) {
                    mn = mn + seed_map.src as i64 - seed_map.dest as i64;
                }
            }
        }
        for range in ranges.iter() {
            if range.contains(&mn) {
                break 'outer;
            }
        }
        ans += 1;
    }
    ans as u64
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

    fn contains_dst(&self, seed: i64) -> bool {
        (self.dest..self.dest + self.range_len).contains(&(seed as usize))
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
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
