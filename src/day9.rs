use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect()
        })
        .collect()
}

fn calc_diffs(line: &[i32]) -> Vec<i32> {
    line.iter()
        .zip(line.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

#[aoc(day9, part1)]
fn part1(lines: &[Vec<i32>]) -> i32 {
    lines
        .iter()
        .map(|line| {
            let mut edges = vec![];
            let mut line = line.clone();
            while line.iter().any(|&x| x != 0) {
                edges.push(*line.last().unwrap());
                line = calc_diffs(&line);
            }
            edges.iter().rev().sum::<i32>()
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(lines: &[Vec<i32>]) -> i32 {
    lines
        .iter()
        .map(|line| {
            let mut edges = vec![];
            let mut line = line.clone();
            while line.iter().any(|&x| x != 0) {
                edges.push(*line.first().unwrap());
                line = calc_diffs(&line);
            }
            edges.iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 114)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 2)
    }
}
