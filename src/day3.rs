use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
struct Symbol {
    x: u32,
    y: u32,
    c: char,
}

#[derive(Debug)]
struct Part {
    y: u32,
    start_x: u32,
    end_x: u32,
    value: u32,
}

impl Part {
    fn adjacent(self: &Self, symbol: &Symbol) -> bool {
        self.y - 1 <= symbol.y
            && symbol.y <= self.y + 1
            && self.start_x <= symbol.x
            && symbol.x <= self.end_x
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> (Vec<Part>, Vec<Symbol>) {
    let mut parts = vec![];
    let mut symbols = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut part = false;
        let mut value = 0u32;
        let mut start_x = 0u32;
        let mut last_x = 0u32;
        for (x, c) in line.chars().enumerate() {
            last_x = x as u32;
            if let Some(v) = c.to_digit(10) {
                if part {
                    value = value * 10 + v;
                } else {
                    start_x = x as u32;
                    value = v;
                    part = true;
                }
            } else {
                if part {
                    parts.push(Part {
                        y: (y + 1) as u32,
                        start_x,
                        end_x: (x + 1) as u32,
                        value,
                    });
                    part = false;
                }
                if c != '.' {
                    symbols.push(Symbol {
                        x: (x + 1) as u32,
                        y: (y + 1) as u32,
                        c,
                    });
                }
            }
        }
        if part {
            parts.push(Part {
                y: (y + 1) as u32,
                start_x,
                end_x: (last_x + 1) as u32,
                value,
            });
        }
    }
    (parts, symbols)
}

#[aoc(day3, part1)]
fn part1(stuff: &(Vec<Part>, Vec<Symbol>)) -> u32 {
    let (parts, symbols) = stuff;
    parts
        .iter()
        .filter_map(|part| {
            symbols
                .iter()
                .any(|symbol| part.adjacent(symbol))
                .then(|| part.value)
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(stuff: &(Vec<Part>, Vec<Symbol>)) -> u32 {
    let (parts, symbols) = stuff;
    symbols
        .iter()
        .filter(|symbol| symbol.c == '*')
        .filter(|symbol| parts.iter().filter(|part| part.adjacent(symbol)).count() == 2)
        .map(|symbol| {
            parts
                .iter()
                .filter(|part| part.adjacent(symbol))
                .map(|part| part.value)
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 4361)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 467835)
    }
}
