use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct SeedRange {
    start: u64,
    len: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        self.src_start <= num && num < self.src_start + self.len
    }

    fn convert(&self, num: u64) -> u64 {
        num - self.src_start + self.dst_start
    }
}

impl SeedRange {
    fn split_map(&self, range: &Range) -> (SeedRange, Option<SeedRange>) {
        let last = self.start + self.len - 1;
        let range_last = range.src_start + range.len;
        if last <= range.src_start + range.len {
            (
                SeedRange {
                    start: range.convert(self.start),
                    len: self.len,
                },
                None,
            )
        } else {
            let len = range_last - self.start;
            (
                SeedRange {
                    start: range.convert(self.start),
                    len,
                },
                Some(SeedRange {
                    start: self.start + len,
                    len: self.len - len,
                }),
            )
        }
    }
}

impl Map {
    fn convert(&self, num: u64) -> u64 {
        for range in &self.ranges {
            if range.contains(num) {
                return range.convert(num);
            }
        }
        num
    }

    fn convert_range(&self, seeds: &SeedRange) -> Vec<SeedRange> {
        let mut result = vec![];
        for range in &self.ranges {
            if range.contains(seeds.start) {
                match seeds.split_map(range) {
                    (first, Some(rem)) => {
                        result.push(first);
                        result.extend(self.convert_range(&rem));
                    }
                    (first, None) => {
                        result.push(first);
                    }
                }
            }
        }
        if result.is_empty() {
            result.push(*seeds)
        }
        result
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
fn input_generator(input: &str) -> Input {
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
fn part1(stuff: &Input) -> u64 {
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

#[aoc(day5, part2)]
fn part2(stuff: &Input) -> u64 {
    let (seeds, names, maps) = stuff;
    seeds
        .chunks(2)
        .map(|c| {
            let start = *c.first().unwrap();
            let len = *c.get(1).unwrap();
            SeedRange { start, len }
        })
        .map(|seed_range| {
            let mut src_name = "seed";
            let mut ranges = vec![seed_range];
            loop {
                let dst_name = names.get(src_name).unwrap();
                let map = maps.get(dst_name).unwrap();
                let mut new_ranges = vec![];
                for range in &ranges {
                    let new = map.convert_range(range);
                    new_ranges.extend(new);
                }
                ranges = new_ranges;
                src_name = dst_name;
                if src_name == "location" {
                    break;
                }
            }
            *ranges
                .iter()
                .map(|SeedRange { start, .. }| start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

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

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 35)
    }

    #[test]
    fn split_seed_range() {
        assert_eq!(
            SeedRange { start: 55, len: 13 }.split_map(&Range {
                dst_start: 52,
                src_start: 50,
                len: 48
            }),
            (SeedRange { start: 57, len: 13 }, None)
        )
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 46)
    }
}
