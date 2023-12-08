use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

fn parse_instruction(c: char) -> Instruction {
    match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!(),
    }
}

fn parse_node(line: &str) -> (String, (String, String)) {
    let (name, rest) = line.split_once(" = ").unwrap();
    let rest = rest.trim_matches(|c| c == '(' || c == ')');
    let (left, right) = rest.split_once(", ").unwrap();
    (name.to_string(), (left.to_string(), right.to_string()))
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instrs = lines
        .next()
        .unwrap()
        .chars()
        .map(parse_instruction)
        .collect();
    lines.next();
    let map = lines.map(parse_node).collect();
    (instrs, map)
}

#[aoc(day8, part1)]
fn part1(input: &(Vec<Instruction>, HashMap<String, (String, String)>)) -> usize {
    let (instrs, map) = input;
    let mut node = "AAA";
    instrs
        .iter()
        .cycle()
        .enumerate()
        .find(|(_, instr)| {
            node = match instr {
                Instruction::Left => &map.get(node).unwrap().0,
                Instruction::Right => &map.get(node).unwrap().1,
            };
            node == "ZZZ"
        })
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input_generator(input)), 2);
    }

    #[test]
    fn sample2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input_generator(input)), 6);
    }
}
