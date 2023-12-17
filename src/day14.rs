use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

#[derive(Clone)]
enum Tile {
    Empty,
    Cube,
    Rock,
}

type Pos = (usize, usize);
type Map = HashMap<Pos, Tile>;
type Rock = Pos;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> (Map, Vec<Rock>, (usize, usize)) {
    let mut map = HashMap::new();
    let mut rocks = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in input.lines().enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            max_x = x;
            match c {
                '.' => {
                    map.insert((x, y), Tile::Empty);
                }
                '#' => {
                    map.insert((x, y), Tile::Cube);
                }
                'O' => {
                    map.insert((x, y), Tile::Empty);
                    rocks.push((x, y));
                }
                _ => panic!(),
            }
        }
    }
    (map, rocks, (max_x + 1, max_y + 1))
}

#[aoc(day14, part1)]
fn part1(input: &(Map, Vec<Rock>, (usize, usize))) -> usize {
    let (mut map, mut rocks, (_width, height)) = input.to_owned();
    for rock in rocks.iter_mut() {
        tilt(&map, rock, |r| r.1 > 0, |(x, y)| (*x, *y - 1), |r| r.1 -= 1);
        map.insert(*rock, Tile::Rock);
    }
    calculate_load(&rocks, height)
}

fn calculate_load(rocks: &[Rock], height: usize) -> usize {
    rocks.iter().map(|(_, y)| height - y).sum()
}

fn show_map(map: &Map, size: (usize, usize)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            match map.get(&(x, y)).unwrap() {
                Tile::Empty => print!("."),
                Tile::Cube => print!("#"),
                Tile::Rock => print!("O"),
            }
        }
        println!();
    }
}

fn tilt<C: Fn(&Rock) -> bool, N: Fn(&Rock) -> Pos, M: Fn(&mut Rock)>(
    map: &Map,
    rock: &mut Rock,
    cond: C,
    next: N,
    modify: M,
) {
    while cond(rock) {
        match map.get(&next(rock)).unwrap() {
            Tile::Empty => modify(rock),
            _ => break,
        }
    }
}

fn spin_cycle(map: &mut Map, rocks: &mut [Rock], size: (usize, usize)) {
    // North
    rocks.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));
    for rock in rocks.iter_mut() {
        map.insert(*rock, Tile::Empty);
        tilt(map, rock, |r| r.1 > 0, |(x, y)| (*x, *y - 1), |r| r.1 -= 1);
        map.insert(*rock, Tile::Rock);
    }
    // West
    rocks.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));
    for rock in rocks.iter_mut() {
        map.insert(*rock, Tile::Empty);
        tilt(map, rock, |r| r.0 > 0, |(x, y)| (*x - 1, *y), |r| r.0 -= 1);
        map.insert(*rock, Tile::Rock);
    }
    // South
    rocks.sort_by(|(_, y1), (_, y2)| y2.cmp(y1));
    for rock in rocks.iter_mut() {
        map.insert(*rock, Tile::Empty);
        tilt(
            map,
            rock,
            |r| r.1 < size.1 - 1,
            |(x, y)| (*x, *y + 1),
            |r| r.1 += 1,
        );
        map.insert(*rock, Tile::Rock);
    }
    // East
    rocks.sort_by(|(x1, _), (x2, _)| x2.cmp(x1));
    for rock in rocks.iter_mut() {
        map.insert(*rock, Tile::Empty);
        tilt(
            map,
            rock,
            |r| r.0 < size.0 - 1,
            |(x, y)| (*x + 1, *y),
            |r| r.0 += 1,
        );
        map.insert(*rock, Tile::Rock);
    }
}

#[aoc(day14, part2)]
fn part2(input: &(Map, Vec<Rock>, (usize, usize))) -> usize {
    let (mut map, mut rocks, (width, height)) = input.to_owned();
    let mut memory = HashMap::<Vec<Rock>, usize>::new();
    const GOAL: usize = 1_000_000_000;
    let mut remainder = None;
    for i in 1..=GOAL {
        spin_cycle(&mut map, &mut rocks, (width, height));
        if memory.contains_key(&rocks) {
            let loop_length = i - memory.get(&rocks).unwrap();
            remainder = Some((GOAL - i) % loop_length);
            break;
        } else {
            memory.insert(rocks.clone(), i);
        }
    }
    if let Some(remainder) = remainder {
        for _ in 1..=remainder {
            spin_cycle(&mut map, &mut rocks, (width, height));
        }
    }
    calculate_load(&rocks, height)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(&input_generator(input)), 136);
    }

    #[test]
    fn sample2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part2(&input_generator(input)), 64);
    }
}
