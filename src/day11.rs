use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashSet;

type Pos = (usize, usize);

#[aoc_generator(day11)]
fn input_generator(input: &str) -> (Vec<Pos>, Vec<usize>, Vec<usize>) {
    let input_width = input.lines().next().unwrap().len();
    let mut empty_columns = (0..input_width).collect::<HashSet<usize>>();
    let mut empty_rows = vec![];
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let row_galaxies = line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => {
                        empty_columns.remove(&x);
                        Some((x, y))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            if row_galaxies.is_empty() {
                empty_rows.push(y);
            }
            row_galaxies
        })
        .collect();
    let mut empty_columns = Vec::from_iter(empty_columns);
    empty_columns.sort();
    (galaxies, empty_rows, empty_columns)
}

fn real_coords(factor: usize, galaxy: &Pos, empty_rows: &[usize], empty_columns: &[usize]) -> Pos {
    (
        galaxy.0 + (factor - 1) * empty_columns.iter().filter(|&cx| *cx < galaxy.0).count(),
        galaxy.1 + (factor - 1) * empty_rows.iter().filter(|&ry| *ry < galaxy.1).count(),
    )
}

fn expand_space(factor: usize, input: &(Vec<Pos>, Vec<usize>, Vec<usize>)) -> Vec<Pos> {
    let (galaxies, empty_rows, empty_columns) = input;
    galaxies
        .iter()
        .map(|g| real_coords(factor, g, empty_rows, empty_columns))
        .collect()
}

fn sum_distances(galaxies: &[Pos]) -> usize {
    let mut distance = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            distance += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
        }
    }
    distance
}

#[aoc(day11, part1)]
fn part1(input: &(Vec<Pos>, Vec<usize>, Vec<usize>)) -> usize {
    sum_distances(&expand_space(2, input))
}

#[aoc(day11, part2)]
fn part2(input: &(Vec<Pos>, Vec<usize>, Vec<usize>)) -> usize {
    sum_distances(&expand_space(1_000_000, input))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 374)
    }

    #[test]
    fn sample2() {
        assert_eq!(
            sum_distances(&expand_space(10, &input_generator(INPUT))),
            1030
        );
        assert_eq!(
            sum_distances(&expand_space(100, &input_generator(INPUT))),
            8410
        )
    }
}
