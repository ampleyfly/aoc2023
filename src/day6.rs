use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day6, part1)]
fn input_generator_part1(input: &str) -> Vec<(u64, u64)> {
    let mut input = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).map(|n| n.parse().unwrap()));
    let durations = input.next().unwrap();
    let records = input.next().unwrap();
    durations.zip(records).collect()
}

#[aoc_generator(day6, part2)]
fn input_generator_part2(input: &str) -> (u64, u64) {
    let mut input = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    });
    let duration = input.next().unwrap();
    let record = input.next().unwrap();
    (duration, record)
}

fn count_ways_to_win(race: &(u64, u64)) -> u64 {
    let (duration, record) = (race.0 as f64, race.1 as f64);
    let diff = (duration * duration / 4.0 - record).sqrt();
    let minf = duration / 2.0 - diff;
    let maxf = duration / 2.0 + diff;
    let min = minf.ceil() as u64;
    let max = maxf.floor() as u64;
    let mut count = max - min + 1;
    if minf == (minf as u64) as f64 {
        count -= 1;
    }
    if maxf == (maxf as u64) as f64 {
        count -= 1;
    }
    count
}

#[aoc(day6, part1)]
fn part1(races: &[(u64, u64)]) -> u64 {
    races.iter().map(count_ways_to_win).product()
}

#[aoc(day6, part2)]
fn part2(race: &(u64, u64)) -> u64 {
    count_ways_to_win(race)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator_part1(INPUT)), 288)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator_part2(INPUT)), 71503)
    }
}
