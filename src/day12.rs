use aoc_runner_derive::aoc;

use std::collections::HashMap;

type Slot<'a> = &'a str;
type Row<'a> = Vec<Slot<'a>>;
type GroupSize = usize;

fn input_generator<'a>(input: &'a str) -> Vec<(Row<'a>, Vec<GroupSize>)> {
    input
        .lines()
        .map(|line| {
            let (row, groups) = line.split_once(' ').unwrap();
            let groups = groups.split(',').map(|s| s.parse().unwrap()).collect();
            let row = row.split('.').filter(|r| !r.is_empty()).collect();
            (row, groups)
        })
        .collect()
}

fn input_generator_part2(input: &str) -> Vec<(Vec<String>, Vec<GroupSize>)> {
    input
        .lines()
        .map(|line| {
            let (row, groups) = line.split_once(' ').unwrap();
            let groups: Vec<GroupSize> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            let groups = vec![groups].into_iter().cycle().take(5).flatten().collect();
            let row: String = vec![row]
                .into_iter()
                .cycle()
                .take(5)
                .intersperse("?")
                .collect();
            let row = row
                .split('.')
                .filter(|r| !r.is_empty())
                .map(|s| s.to_string())
                .collect();
            (row, groups)
        })
        .collect()
}

fn placements<'a, 'b>(
    slot: &'a Slot<'b>,
    group_size: &'a GroupSize,
) -> impl Iterator<Item = Option<Slot<'b>>> + 'a {
    let last_start = match slot.chars().position(|s| s == '#') {
        Some(i) => i.min(slot.len() - group_size),
        None => slot.len() - group_size,
    };
    (0..=last_start)
        .filter(move |start| {
            start + group_size == slot.len() || slot.chars().nth(start + group_size).unwrap() == '?'
        })
        .map(move |start| {
            if start + group_size + 1 >= slot.len() {
                None
            } else {
                Some(&slot[start + group_size + 1..])
            }
        })
}

type Memory<'a> = HashMap<(Vec<Slot<'a>>, Vec<GroupSize>), usize>;

fn recall(memory: &Memory, slots: &[Slot], groups: &[GroupSize]) -> Option<usize> {
    memory.get(&(slots.to_vec(), groups.to_vec())).copied()
}

fn record<'a>(memory: &mut Memory<'a>, slots: &[Slot<'a>], groups: &[GroupSize], result: usize) {
    memory.insert((slots.to_owned(), groups.to_owned()), result);
}

fn count_arrangements<'a, 'b>(
    slots: &'b [Slot<'a>],
    groups: &'b [GroupSize],
    memory: &mut Memory<'a>,
) -> usize {
    if let Some(result) = recall(memory, slots, groups) {
        return result;
    }
    let result = if groups.is_empty() {
        if slots.iter().all(|slot| !slot.chars().any(|s| s == '#')) {
            // No more groups to place and no more damaged springs to cover, so this arrangement counts
            1
        } else {
            // Ran out of groups to arrange, but there are still damaged springs to cover
            0
        }
    } else if slots.is_empty() {
        // Ran out of slots to arrange remaining groups in
        0
    } else {
        let skippable_slot = !slots[0].chars().any(|s| s == '#');
        if groups[0] > slots[0].len() {
            if skippable_slot {
                // Skip this slot
                count_arrangements(&slots[1..], groups, memory)
            } else {
                // This arrangement cannot cover the damaged springs in this slot
                0
            }
        } else if groups[0] == slots[0].len() {
            // Try both using and skipping this slot
            count_arrangements(&slots[1..], &groups[1..], memory)
                + if skippable_slot {
                    count_arrangements(&slots[1..], groups, memory)
                } else {
                    0
                }
        } else {
            // Try every possible placement of the group within the slot, as well as skipping the slot, if applicable
            placements(&slots[0], &groups[0])
                .map(|slot| match slot {
                    Some(slot) => {
                        let mut rem = vec![slot];
                        rem.extend_from_slice(&slots[1..]);
                        count_arrangements(&rem, &groups[1..], memory)
                    }
                    None => count_arrangements(&slots[1..], &groups[1..], memory),
                })
                .sum::<usize>()
                + if skippable_slot {
                    count_arrangements(&slots[1..], groups, memory)
                } else {
                    0
                }
        }
    };
    record(memory, slots, groups, result);
    result
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let input = input_generator(input);
    input
        .iter()
        .map(|(slots, groups)| count_arrangements(slots, groups, &mut HashMap::new()))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let input = input_generator_part2(input);
    input
        .iter()
        .map(|(slots, groups)| {
            let slots = slots
                .iter()
                .map(|slot| slot.as_str())
                .collect::<Vec<&str>>();
            count_arrangements(&slots, groups, &mut HashMap::new())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn sample1() {
        assert_eq!(part1(INPUT.lines().nth(0).unwrap()), 1);
        assert_eq!(part1(INPUT.lines().nth(1).unwrap()), 4);
        assert_eq!(part1(INPUT.lines().nth(2).unwrap()), 1);
        assert_eq!(part1(INPUT.lines().nth(3).unwrap()), 1);
        assert_eq!(part1(INPUT.lines().nth(4).unwrap()), 4);
        assert_eq!(part1(INPUT.lines().nth(5).unwrap()), 10);
        assert_eq!(part1(INPUT), 21)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(INPUT.lines().nth(0).unwrap()), 1);
        assert_eq!(part2(INPUT.lines().nth(1).unwrap()), 16384);
        assert_eq!(part2(INPUT.lines().nth(2).unwrap()), 1);
        assert_eq!(part2(INPUT.lines().nth(3).unwrap()), 16);
        assert_eq!(part2(INPUT.lines().nth(4).unwrap()), 2500);
        assert_eq!(part2(INPUT.lines().nth(5).unwrap()), 506250);
        assert_eq!(part2(INPUT), 525152)
    }
}
