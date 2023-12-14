use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use maplit::hashmap;

#[aoc_generator(day1, part1)]
fn input_part1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect()
}

fn rev_inits(line: &str) -> impl Iterator<Item = &str> {
    (1..=line.len()).rev().map(|end| &line[..end])
}

fn tails(line: &str) -> impl Iterator<Item = &str> {
    (0..line.len()).map(|start| &line[start..])
}

fn find_first_digit(line: &str) -> u32 {
    let numbers = hashmap! {
        "one" => 1u32,
        "two" => 2u32,
        "three" => 3u32,
        "four" => 4u32,
        "five" => 5u32,
        "six" => 6u32,
        "seven" => 7u32,
        "eight" => 8u32,
        "nine" => 9u32,
    };
    tails(line)
        .find_map(|tail| {
            tail.chars().next().unwrap().to_digit(10).or_else(|| {
                numbers
                    .keys()
                    .find_map(|&key| tail.starts_with(key).then(|| numbers.get(key).unwrap()))
                    .copied()
            })
        })
        .unwrap()
}

fn find_last_digit(line: &str) -> u32 {
    let numbers = hashmap! {
        "one" => 1u32,
        "two" => 2u32,
        "three" => 3u32,
        "four" => 4u32,
        "five" => 5u32,
        "six" => 6u32,
        "seven" => 7u32,
        "eight" => 8u32,
        "nine" => 9u32,
    };
    rev_inits(line)
        .find_map(|init| {
            init.chars().last().unwrap().to_digit(10).or_else(|| {
                numbers
                    .keys()
                    .find_map(|&key| init.ends_with(key).then(|| numbers.get(key).unwrap()))
                    .copied()
            })
        })
        .unwrap()
}

#[aoc_generator(day1, part2)]
fn input_part2(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let first = find_first_digit(line);
            let last = find_last_digit(line);
            vec![first, last]
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(lines: &[Vec<u32>]) -> String {
    lines
        .iter()
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum::<u32>()
        .to_string()
}

#[aoc(day1, part2)]
fn part2(lines: &[Vec<u32>]) -> String {
    part1(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "1abc2
                     pqr3stu8vwx
                     a1b2c3d4e5f
                     treb7uchet";
        assert_eq!(part1(&input_part1(input)), "142")
    }

    #[test]
    fn sample2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&input_part2(input)), "281")
    }
}
