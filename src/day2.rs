use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::cmp::max;

struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl Subset {
    fn contains(self: &Self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(self: &Self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<(u32, Vec<Subset>)> {
    input
        .lines()
        .map(|line| {
            let (game, subsets) = line.split_once(": ").unwrap();
            let game: u32 = game.split_once(' ').unwrap().1.parse().unwrap();
            let subsets = subsets
                .split("; ")
                .map(|subset| {
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    for count in subset.split(", ") {
                        match count.split_once(' ').unwrap() {
                            (c, "red") => red = c.parse().unwrap(),
                            (c, "green") => green = c.parse().unwrap(),
                            (c, "blue") => blue = c.parse().unwrap(),
                            _ => panic!("Not matched: {}", count),
                        }
                    }
                    Subset { red, green, blue }
                })
                .collect();
            (game, subsets)
        })
        .collect()
}

fn union(sets: &Vec<Subset>) -> Subset {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for set in sets {
        red = max(red, set.red);
        green = max(green, set.green);
        blue = max(blue, set.blue);
    }
    Subset { red, green, blue }
}

#[aoc(day2, part1)]
fn part1(games: &Vec<(u32, Vec<Subset>)>) -> u32 {
    let actual = Subset {
        red: 12,
        green: 13,
        blue: 14,
    };
    games
        .iter()
        .filter_map(|(game, subsets)| actual.contains(&union(subsets)).then(|| game))
        .sum()
}

#[aoc(day2, part2)]
fn part2(games: &Vec<(u32, Vec<Subset>)>) -> u32 {
    games
        .iter()
        .map(|(_game, subsets)| union(subsets).power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 8)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 2286)
    }
}
