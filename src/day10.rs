use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (usize, usize);
type Size = (usize, usize);

struct Loc {
    pos: Pos,
    dir: Dir,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Dir {
    Right = 1,
    Down = 4,
    Left = 16,
    Up = 64,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Tile {
    LeftRight = Dir::Left as u8 | Dir::Right as u8,
    UpDown = Dir::Up as u8 | Dir::Down as u8,
    UpRight = Dir::Up as u8 | Dir::Right as u8,
    UpLeft = Dir::Up as u8 | Dir::Left as u8,
    DownLeft = Dir::Down as u8 | Dir::Left as u8,
    DownRight = Dir::Down as u8 | Dir::Right as u8,
    Empty = 0,
}

fn parse_tile(c: char) -> Option<Tile> {
    match c {
        '-' => Some(Tile::LeftRight),
        '|' => Some(Tile::UpDown),
        'L' => Some(Tile::UpRight),
        'J' => Some(Tile::UpLeft),
        '7' => Some(Tile::DownLeft),
        'F' => Some(Tile::DownRight),
        '.' => Some(Tile::Empty),
        'S' => None,
        _ => panic!(),
    }
}

fn just_dir(tile: Tile, dir: Dir) -> u8 {
    (tile as u8) & (dir as u8)
}

fn has_dir(tile: Tile, dir: Dir) -> bool {
    just_dir(tile, dir) != 0
}

fn flip_tile(tile: Tile) -> Tile {
    to_tile((tile as u8).rotate_left(4))
}

fn flip_dir(dir: Dir) -> Dir {
    to_dir((dir as u8).rotate_left(4))
}

fn to_dir(val: u8) -> Dir {
    unsafe {
        assert!(
            val == Dir::Right as u8
                || val == Dir::Down as u8
                || val == Dir::Left as u8
                || val == Dir::Up as u8
        );
        std::mem::transmute(val)
    }
}

fn to_tile(val: u8) -> Tile {
    unsafe {
        assert!(
            val == Tile::LeftRight as u8
                || val == Tile::UpDown as u8
                || val == Tile::UpRight as u8
                || val == Tile::UpLeft as u8
                || val == Tile::DownLeft as u8
                || val == Tile::DownRight as u8
                || val == Tile::Empty as u8
        );
        std::mem::transmute(val)
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> (Pos, Size, HashMap<Pos, Tile>) {
    let mut start = (0, 0);
    let mut tiles = HashMap::new();
    let mut size = (0, 0);
    for (y, line) in input.lines().enumerate() {
        size.1 = y + 1;
        for (x, c) in line.chars().enumerate() {
            size.0 = x + 1;
            match parse_tile(c) {
                None => {
                    start = (x + 1, y + 1);
                }
                Some(tile) => {
                    tiles.insert((x + 1, y + 1), tile);
                }
            }
        }
    }
    for y in 1..=size.1 {
        tiles.insert((0, y), Tile::Empty);
        tiles.insert((size.0 + 1, y), Tile::Empty);
    }
    for x in 0..=size.0 + 1 {
        tiles.insert((x, 0), Tile::Empty);
        tiles.insert((x, size.1 + 1), Tile::Empty);
    }
    let up = just_dir(tiles[&(start.0, start.1 - 1)], Dir::Down);
    let down = just_dir(tiles[&(start.0, start.1 + 1)], Dir::Up);
    let left = just_dir(tiles[&(start.0 - 1, start.1)], Dir::Right);
    let right = just_dir(tiles[&(start.0 + 1, start.1)], Dir::Left);
    let start_tile = flip_tile(to_tile(up | down | left | right));
    tiles.insert(start, start_tile);
    (start, (size.0 + 2, size.1 + 2), tiles)
}

fn step(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::Up => (pos.0, pos.1 - 1),
        Dir::Down => (pos.0, pos.1 + 1),
        Dir::Left => (pos.0 - 1, pos.1),
        Dir::Right => (pos.0 + 1, pos.1),
    }
}

fn turn(tile: Tile, dir: Dir) -> Dir {
    to_dir((tile as u8) ^ (flip_dir(dir) as u8))
}

fn walk_cycle(start: &Pos, tiles: &HashMap<Pos, Tile>) -> (HashSet<Pos>, usize) {
    let mut loc = Loc {
        pos: *start,
        dir: *[Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .iter()
            .find(|&dir| has_dir(tiles[&start], *dir))
            .unwrap(),
    };
    let mut visited = HashSet::new();
    let steps = (1..)
        .find(|_| {
            visited.insert(loc.pos);
            loc.pos = step(loc.pos, loc.dir);
            loc.dir = turn(tiles[&loc.pos], loc.dir);
            visited.contains(&loc.pos)
        })
        .unwrap();
    (visited, steps)
}

#[aoc(day10, part1)]
fn part1(input: &(Pos, Size, HashMap<Pos, Tile>)) -> usize {
    let (start, _, tiles) = input;
    let (_, steps) = walk_cycle(start, tiles);
    steps / 2
}

#[derive(PartialEq)]
enum State {
    UpperEdge,
    LowerEdge,
    Inside,
    Outside,
}

#[aoc(day10, part2)]
fn part2(input: &(Pos, Size, HashMap<Pos, Tile>)) -> usize {
    let (start, size, tiles) = input;
    let (visited, _) = walk_cycle(start, tiles);
    let mut count = 0;
    for y in 1..size.1 - 1 {
        let mut state = State::Outside;
        for x in 1..size.0 - 1 {
            if !visited.contains(&(x, y)) {
                if state == State::Inside {
                    count += 1;
                }
                continue;
            }
            state = match (&state, tiles[&(x, y)]) {
                (State::Outside, Tile::UpDown) => State::Inside,
                (State::Outside, Tile::DownRight) => State::UpperEdge,
                (State::Outside, Tile::UpRight) => State::LowerEdge,
                (State::Outside, _) => state,
                (State::Inside, Tile::UpDown) => State::Outside,
                (State::Inside, Tile::UpRight) => State::UpperEdge,
                (State::Inside, Tile::DownRight) => State::LowerEdge,
                (State::Inside, _) => State::Outside,
                (State::UpperEdge, Tile::UpLeft) => State::Inside,
                (State::UpperEdge, Tile::DownLeft) => State::Outside,
                (State::LowerEdge, Tile::DownLeft) => State::Inside,
                (State::LowerEdge, Tile::UpLeft) => State::Outside,
                (_, Tile::LeftRight) => state,
                _ => panic!(),
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(&input_generator(input)), 4)
    }

    #[test]
    fn sample2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(&input_generator(input)), 8)
    }

    #[test]
    fn sample3() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(&input_generator(input)), 4)
    }

    #[test]
    fn sample4() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(part2(&input_generator(input)), 4)
    }

    #[test]
    fn sample5() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(&input_generator(input)), 8)
    }

    #[test]
    fn sample6() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(&input_generator(input)), 10)
    }
}
