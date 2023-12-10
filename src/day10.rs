use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::HashSet;

type Dir = u8;
type Tile = u8;
type Pos = (usize, usize);
type Size = (usize, usize);

struct Loc {
    pos: Pos,
    dir: Dir,
}

mod Dirs {
    pub const Right: u8 = 1;
    pub const Down: u8 = 4;
    pub const Left: u8 = 16;
    pub const Up: u8 = 64;
}

mod Tiles {
    use super::Dirs;

    pub const LeftRight: u8 = Dirs::Left | Dirs::Right;
    pub const UpDown: u8 = Dirs::Up | Dirs::Down;
    pub const UpRight: u8 = Dirs::Up | Dirs::Right;
    pub const UpLeft: u8 = Dirs::Up | Dirs::Left;
    pub const DownLeft: u8 = Dirs::Down | Dirs::Left;
    pub const DownRight: u8 = Dirs::Down | Dirs::Right;
    pub const Empty: u8 = 0;
}

fn parse_tile(c: char) -> Option<Tile> {
    match c {
        '-' => Some(Tiles::LeftRight),
        '|' => Some(Tiles::UpDown),
        'L' => Some(Tiles::UpRight),
        'J' => Some(Tiles::UpLeft),
        '7' => Some(Tiles::DownLeft),
        'F' => Some(Tiles::DownRight),
        '.' => Some(Tiles::Empty),
        'S' => None,
        _ => panic!(),
    }
}

fn flip_tile(tile: Tile) -> Tile {
    tile.rotate_left(4)
}

fn flip_dir(dir: Dir) -> Dir {
    dir.rotate_left(4)
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
        tiles.insert((0, y), Tiles::Empty);
        tiles.insert((size.0 + 1, y), Tiles::Empty);
    }
    for x in 0..=size.0 + 1 {
        tiles.insert((x, 0), Tiles::Empty);
        tiles.insert((x, size.1 + 1), Tiles::Empty);
    }
    let up = tiles[&(start.0, start.1 - 1)] & Dirs::Down;
    let down = tiles[&(start.0, start.1 + 1)] & Dirs::Up;
    let left = tiles[&(start.0 - 1, start.1)] & Dirs::Right;
    let right = tiles[&(start.0 + 1, start.1)] & Dirs::Left;
    let start_tile = flip_tile(up | down | left | right);
    tiles.insert(start, start_tile);
    (start, (size.0 + 2, size.1 + 2), tiles)
}

fn step(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dirs::Up => (pos.0, pos.1 - 1),
        Dirs::Down => (pos.0, pos.1 + 1),
        Dirs::Left => (pos.0 - 1, pos.1),
        Dirs::Right => (pos.0 + 1, pos.1),
        _ => panic!(),
    }
}

fn turn(tile: Tile, dir: Dir) -> Dir {
    tile ^ flip_dir(dir)
}

fn show_tile(tile: Tile) -> char {
    match tile {
        Tiles::LeftRight => '─',
        Tiles::UpDown => '│',
        Tiles::UpRight => '└',
        Tiles::UpLeft => '┘',
        Tiles::DownLeft => '┐',
        Tiles::DownRight => '┌',
        Tiles::Empty => '░',
        _ => panic!(),
    }
}

fn show_dir(dir: Dir) -> char {
    match dir {
        Dirs::Up => '↑',
        Dirs::Down => '↓',
        Dirs::Left => '←',
        Dirs::Right => '→',
        _ => panic!(),
    }
}

fn walk_cycle(start: &Pos, tiles: &HashMap<Pos, Tile>) -> (HashSet<Pos>, usize) {
    let mut loc = Loc {
        pos: *start,
        dir: *vec![Dirs::Up, Dirs::Down, Dirs::Left, Dirs::Right]
            .iter()
            .find(|&dir| tiles[&start] & dir != 0)
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

fn show_tiles(size: Size, tiles: &HashMap<Pos, Tile>) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            print!("{}", show_tile(tiles[&(x, y)]));
        }
        println!("");
    }
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
                (State::Outside, Tiles::UpDown) => State::Inside,
                (State::Outside, Tiles::DownRight) => State::UpperEdge,
                (State::Outside, Tiles::UpRight) => State::LowerEdge,
                (State::Outside, _) => state,
                (State::Inside, Tiles::UpDown) => State::Outside,
                (State::Inside, Tiles::UpRight) => State::UpperEdge,
                (State::Inside, Tiles::DownRight) => State::LowerEdge,
                (State::Inside, _) => State::Outside,
                (State::UpperEdge, Tiles::UpLeft) => State::Inside,
                (State::UpperEdge, Tiles::DownLeft) => State::Outside,
                (State::LowerEdge, Tiles::DownLeft) => State::Inside,
                (State::LowerEdge, Tiles::UpLeft) => State::Outside,
                (_, Tiles::LeftRight) => state,
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
