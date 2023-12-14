use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let lines = input.lines().chain([""]);
    let mut patterns = Vec::new();
    let mut rows = Vec::<String>::new();
    for line in lines {
        if line.is_empty() {
            let cols = (0..rows[0].len())
                .map(|i| rows.iter().map(|r| r.chars().nth(i).unwrap()).collect())
                .collect();
            patterns.push((rows, cols));
            rows = Vec::new();
        } else {
            rows.push(line.to_string());
        }
    }
    patterns
}

fn inits<T: AsRef<str>>(line: &T) -> impl Iterator<Item = (usize, &str)> + '_ {
    let line = line.as_ref();
    (2..=line.len()).step_by(2).map(|e| (e / 2, &line[..e]))
}

fn ends<T: AsRef<str>>(line: &T) -> impl Iterator<Item = (usize, &str)> + '_ {
    let line = line.as_ref();
    (0..=line.len() - 2)
        .rev()
        .step_by(2)
        .map(|s| ((s + line.len()) / 2, &line[s..]))
}

fn is_palindrome<T: AsRef<str>>(line: T) -> bool {
    let line = line.as_ref();
    for i in 0..(line.len() / 2) {
        if line.chars().nth(i).unwrap() != line.chars().nth(line.len() - 1 - i).unwrap() {
            return false;
        }
    }
    true
}

fn is_almost_palindrome<T: AsRef<str>>(pos: usize, line: T) -> Option<(usize, bool)> {
    let line = line.as_ref();
    let start = line.chars().take(line.len() / 2);
    let end = line.chars().rev().take(line.len() / 2);
    let diffs: Vec<_> = start
        .zip(end)
        .enumerate()
        .filter(|(_, (s, e))| s != e)
        .map(|(i, _)| i)
        .collect();
    if diffs.len() <= 1 {
        Some((pos, diffs.len() == 1))
    } else {
        None
    }
}

fn find_palindromes<'a>(iter: impl Iterator<Item = (usize, &'a str)>) -> HashSet<usize> {
    iter.filter(|(_, line)| is_palindrome(line))
        .map(|(pos, _)| pos)
        .collect::<HashSet<usize>>()
}

fn find_palindromes_part2<'a>(
    iter: impl Iterator<Item = (usize, &'a str)>,
) -> HashSet<(usize, bool)> {
    iter.filter_map(|(pos, line)| is_almost_palindrome(pos, line))
        .collect::<HashSet<(usize, bool)>>()
}

fn find_mirror<T: AsRef<str>>(lines: &[T]) -> Option<usize> {
    lines
        .iter()
        .map(|line| {
            find_palindromes(inits(line))
                .union(&find_palindromes(ends(line)))
                .cloned()
                .collect()
        })
        .reduce(|acc: HashSet<usize>, s| acc.intersection(&s).cloned().collect())
        .unwrap()
        .iter()
        .next()
        .copied()
}

fn find_mirror_part2<T: AsRef<str>>(lines: &[T]) -> Option<usize> {
    lines
        .iter()
        .map(|line| {
            find_palindromes_part2(inits(line))
                .union(&find_palindromes_part2(ends(line)))
                .cloned()
                .collect::<HashSet<(usize, bool)>>()
        })
        .fold(HashMap::<usize, (usize, bool)>::new(), |mut acc, s| {
            s.iter().for_each(|(pos, diffs)| match diffs {
                false => {
                    acc.entry(*pos)
                        .and_modify(|(e, _)| *e += 1)
                        .or_insert((1, false));
                }
                true => {
                    match acc.get(pos) {
                        None => acc.insert(*pos, (1, true)),
                        Some((e, false)) => acc.insert(*pos, (e + 1, true)),
                        Some((_, true)) => acc.remove(pos),
                    };
                }
            });
            acc
        })
        .iter()
        .filter(|(_, (count, has_diff))| *has_diff && *count == lines.len())
        .map(|(pos, _)| pos)
        .next()
        .copied()
}

#[aoc(day13, part1)]
fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input
        .iter()
        .map(|(rows, cols)| {
            let horizontal_mirror = find_mirror(rows).unwrap_or(0);
            let vertical_mirror = find_mirror(cols).unwrap_or(0);
            100 * vertical_mirror + horizontal_mirror
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input
        .iter()
        .map(|(rows, cols)| {
            let horizontal_mirror = find_mirror_part2(rows).unwrap_or(0);
            let vertical_mirror = find_mirror_part2(cols).unwrap_or(0);
            100 * vertical_mirror + horizontal_mirror
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_palindromes() {
        assert_eq!(find_palindromes(inits(&"abaccaba")), hashset![4]);
        assert_eq!(find_palindromes(ends(&"abaccaba")), hashset![4]);
        assert_eq!(find_mirror(&["abaccaba"]), Some(4));

        assert_eq!(
            find_palindromes_part2(inits(&"#.##..##.")),
            hashset![(1, true), (2, true), (3, true)]
        );
        assert_eq!(
            find_palindromes_part2(ends(&"#.##..##.")),
            hashset![(8, true), (7, false), (5, false)]
        );
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 405);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 400);
    }
}
