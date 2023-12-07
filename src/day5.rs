use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl Range {
    fn contains(self: &Self, num: u64) -> bool {
        self.src_start <= num && num <= self.src_start + self.len
    }

    fn convert(self: &Self, num: u64) -> u64 {
        num - self.src_start + self.dst_start
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn convert(self: &Self, num: u64) -> u64 {
        for range in &self.ranges {
            if range.contains(num) {
                return range.convert(num);
            }
        }
        num
    }
}

type Input = (Vec<u64>, HashMap<String, String>, HashMap<String, Map>);

fn split_map_name(map_name: &str) -> (&str, &str) {
    let mut parts = map_name.split('-');
    let src = parts.next().unwrap();
    parts.next().unwrap();
    let dst = parts.next().unwrap();
    (src, dst)
}

fn parse_map(lines: &mut dyn Iterator<Item = &str>) -> Map {
    let ranges = lines
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut nums = l.split_whitespace().map(|w| w.parse().unwrap());
            let dst_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let len = nums.next().unwrap();
            Range {
                dst_start,
                src_start,
                len,
            }
        })
        .collect();
    Map { ranges }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let mut maps = HashMap::<String, Map>::new();
    let mut names = HashMap::<String, String>::new();
    lines.next();
    while let Some(map_name) = lines.next() {
        let (map_name, _) = map_name.split_once(' ').unwrap();
        let (src_name, dst_name) = split_map_name(map_name);
        let map = parse_map(&mut lines);
        names.insert(src_name.to_string(), dst_name.to_string());
        maps.insert(dst_name.to_string(), map);
    }
    (seeds, names, maps)
}

#[aoc(day5, part1)]
pub fn part1(stuff: &Input) -> u64 {
    let (seeds, names, maps) = stuff;
    seeds
        .iter()
        .map(|seed| {
            let mut src_name = "seed";
            let mut number = *seed;
            loop {
                let dst_name = names.get(src_name).unwrap();
                let map = maps.get(dst_name).unwrap();
                number = map.convert(number);
                src_name = dst_name;
                if src_name == "location" {
                    break;
                }
            }
            number
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::{input_generator, part1};

    fn sample1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(&input_generator(input)), 35)
    }
}
