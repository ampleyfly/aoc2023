use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .map(|line| {
            let (winning, actual) = line.split_once('|').unwrap();
            let winning = winning
                .split_whitespace()
                .skip(2)
                .map(|s| s.parse().unwrap())
                .collect();
            let actual = actual
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (winning, actual)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(cards: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    cards
        .iter()
        .map(
            |(winning, actual)| match winning.intersection(actual).count() {
                0 => 0,
                n => 2u32.pow((n - 1) as u32),
            },
        )
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    let mut counts = HashMap::<usize, u32>::new();
    cards
        .iter()
        .enumerate()
        .map(|(cardnum, (winning, actual))| {
            let matches = winning.intersection(actual).count();
            let count = 1 + *counts.get(&cardnum).unwrap_or(&0);
            for n in cardnum + 1..=cardnum + matches {
                counts.entry(n).and_modify(|c| *c += count).or_insert(count);
            }
            count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 13)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 30)
    }
}
