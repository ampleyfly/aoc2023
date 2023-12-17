use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.to_owned())
        .collect()
}

fn hash(string: &str) -> usize {
    string
        .as_ascii()
        .unwrap()
        .iter()
        .fold(0u8, |cur, c| cur.wrapping_add(c.to_u8()).wrapping_mul(17)) as usize
}

#[aoc(day15, part1)]
fn part1(strings: &[String]) -> usize {
    strings.iter().map(|s| hash(s)).sum()
}

fn remove(map: &mut [Vec<(&str, usize)>], label: &str) {
    let bin_index = hash(label);
    if let Some(index) = map[bin_index].iter().position(|(l, _)| l == &label) {
        map[bin_index].remove(index);
    }
}

fn insert<'a>(map: &mut [Vec<(&'a str, usize)>], label: &'a str, focal_length: usize) {
    let bin_index = hash(label);
    if let Some(index) = map[bin_index].iter().position(|(l, _)| l == &label) {
        map[bin_index][index] = (label, focal_length);
    } else {
        map[bin_index].push((label, focal_length));
    }
}

fn print_boxes(map: &[Vec<(&str, usize)>]) {
    for (bin_index, bin) in map.iter().enumerate() {
        if !bin.is_empty() {
            print!("Box {}:", bin_index);
            for (label, focal_length) in bin {
                print!(" [{} {}]", label, focal_length);
            }
            println!();
        }
    }
    println!();
}

#[aoc(day15, part2)]
fn part2(strings: &[String]) -> usize {
    let mut map = vec![vec![]; 256];
    strings.iter().for_each(|s| {
        if s.ends_with('-') {
            let label = &s[..s.len() - 1];
            remove(&mut map, label);
        } else {
            let (label, focal_length) = s.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            insert(&mut map, label, focal_length);
        }
    });
    map.iter()
        .enumerate()
        .map(|(bin_index, bin)| {
            bin.iter()
                .enumerate()
                .map(|(slot_index, (_, focal_length))| {
                    (bin_index + 1) * (slot_index + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 1320);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(INPUT)), 145);
    }
}
