use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
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

#[aoc(day8, part2, simplified)]
fn part2_simplified(input: &(Vec<Instruction>, HashMap<String, (String, String)>)) -> usize {
    let (instrs, map) = input;
    let mut ghosts: HashMap<usize, &str> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.as_str())
        .enumerate()
        .collect();
    let ghost_count = ghosts.len();
    let mut goals: Vec<Vec<usize>> = vec![vec![]; ghost_count];
    let mut visited: Vec<HashMap<(usize, &str), usize>> = vec![HashMap::new(); ghost_count];
    let mut loops: HashMap<usize, (usize, usize)> = HashMap::new();
    instrs.iter().cycle().enumerate().find(|&(step, instr)| {
        ghosts = ghosts
            .iter()
            .filter_map(|(&index, &node)| {
                let stepmod = step % instrs.len();
                if node.ends_with('Z') {
                    goals[index].push(step);
                }
                visited[index].insert((stepmod, node), step);
                let new = match instr {
                    Instruction::Left => map.get(node).unwrap().0.as_str(),
                    Instruction::Right => map.get(node).unwrap().1.as_str(),
                };
                if visited[index].contains_key(&(stepmod + 1, new)) {
                    loops.insert(
                        index,
                        (
                            *visited[index].get(&(stepmod + 1, new)).unwrap(),
                            step - stepmod,
                        ),
                    );
                    None
                } else {
                    Some((index, new))
                }
            })
            .collect();
        ghosts.is_empty()
    });
    goals.iter().map(|gs| *gs.first().unwrap()).fold(1, lcm)
}

//#[aoc(day8, part2, generic)]
fn part2_generic(input: &(Vec<Instruction>, HashMap<String, (String, String)>)) -> usize {
    let (instrs, map) = input;
    let mut ghosts: HashMap<usize, &str> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.as_str())
        .enumerate()
        .collect();
    let ghost_count = ghosts.len();
    let mut goals: Vec<Vec<usize>> = vec![vec![]; ghost_count];
    let mut visited: Vec<HashMap<(usize, &str), usize>> = vec![HashMap::new(); ghost_count];
    let mut loops: HashMap<usize, (usize, usize)> = HashMap::new();
    instrs.iter().cycle().enumerate().find(|&(step, instr)| {
        ghosts = ghosts
            .iter()
            .filter_map(|(&index, &node)| {
                let stepmod = step % instrs.len();
                if node.ends_with('Z') {
                    goals[index].push(step);
                }
                visited[index].insert((stepmod, node), step);
                let new = match instr {
                    Instruction::Left => map.get(node).unwrap().0.as_str(),
                    Instruction::Right => map.get(node).unwrap().1.as_str(),
                };
                if visited[index].contains_key(&(stepmod + 1, new)) {
                    loops.insert(
                        index,
                        (
                            *visited[index].get(&(stepmod + 1, new)).unwrap(),
                            step - stepmod,
                        ),
                    );
                    None
                } else {
                    Some((index, new))
                }
            })
            .collect();
        ghosts.is_empty()
    });
    let mut goal_steps = (0..ghost_count)
        .map(|index| {
            let (_loop_start, loop_length) = loops[&index];
            let num_goals = goals[index].len();
            goals[index]
                .iter()
                .cycle()
                .enumerate()
                .map(move |(i, goal)| goal + (i / num_goals) * loop_length)
        })
        .collect::<Vec<_>>();
    let mut curs = vec![];
    for goals in &mut goal_steps {
        curs.push(goals.next().unwrap());
    }
    loop {
        let (max_i, &max) = curs.iter().enumerate().max_by_key(|(_, &cur)| cur).unwrap();
        if curs.iter().all(|&cur| cur == max) {
            return max;
        } else {
            for (i, goals) in goal_steps.iter_mut().enumerate() {
                if i == max_i {
                    continue;
                }
                while curs[i] < max {
                    curs[i] = goals.next().unwrap();
                }
            }
        }
    }
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

    #[test]
    fn sample3() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2_simplified(&input_generator(input)), 6);
        assert_eq!(part2_generic(&input_generator(input)), 6);
    }

    #[test]
    fn sample4() {
        let input = "LR

11A = (11Z, XXX)
11B = (11Z, XXX)
11Z = (XXX, 11B)
22A = (22B, XXX)
22B = (22Z, 22Z)
22C = (22B, 22B)
22Z = (22C, 22C)
XXX = (XXX, XXX)";
        assert_eq!(part2_generic(&input_generator(input)), 5);
    }
}
