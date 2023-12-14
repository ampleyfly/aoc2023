use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::cmp::Ordering;
use std::collections::HashMap;

type Card = u32;

#[derive(Debug, Clone, Eq)]
struct Hand {
    bid: u32,
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard(),
    OnePair(),
    TwoPair(),
    ThreeOfAKind(),
    FullHouse(),
    FourOfAKind(),
    FiveOfAKind(),
}

fn count_cards(cards: &Vec<Card>) -> HashMap<Card, u32> {
    let mut counts = HashMap::new();
    for &card in cards {
        counts
            .entry(card)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }
    counts
}

fn reverse_counts(counts: &HashMap<Card, u32>) -> HashMap<u32, Vec<Card>> {
    let mut result = HashMap::<u32, Vec<Card>>::new();
    for (&card, &count) in counts {
        result
            .entry(count)
            .and_modify(|vec| {
                vec.push(card);
            })
            .or_insert(vec![card]);
    }
    result
}

mod part1 {
    use super::*;

    fn classify_hand(counts: &HashMap<u32, Vec<Card>>) -> HandType {
        if counts.contains_key(&5) {
            HandType::FiveOfAKind()
        } else if counts.contains_key(&4) {
            HandType::FourOfAKind()
        } else if counts.contains_key(&3) && counts.contains_key(&2) {
            HandType::FullHouse()
        } else if counts.contains_key(&3) {
            HandType::ThreeOfAKind()
        } else if let Some(two) = counts.get(&2)
            && two.len() == 2
        {
            HandType::TwoPair()
        } else if counts.contains_key(&2) {
            HandType::OnePair()
        } else {
            HandType::HighCard()
        }
    }

    impl Hand {
        fn new(cards: Vec<Card>, bid: u32) -> Self {
            let counts = count_cards(&cards);
            let counts = reverse_counts(&counts);
            let hand_type = classify_hand(&counts);
            Self {
                cards,
                bid,
                hand_type,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.hand_type == other.hand_type && self.cards == other.cards
        }
    }

    fn compare_cards(cards1: &[Card], cards2: &[Card]) -> Ordering {
        let result = match (cards1.first(), cards2.first()) {
            (Some(x), Some(y)) if x == y => compare_cards(&cards1[1..], &cards2[1..]),
            (Some(x), Some(y)) if x > y => Ordering::Greater,
            (Some(x), Some(y)) if x < y => Ordering::Less,
            _ => Ordering::Equal,
        };
        result
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type.cmp(&other.hand_type) {
                Ordering::Equal => compare_cards(&self.cards, &other.cards),
                other => other,
            }
        }
    }

    fn parse_card(c: char) -> Card {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        }
    }

    pub fn parse_hand(line: &str) -> Hand {
        let (hand, bid) = line.split_once(' ').unwrap();
        Hand::new(hand.chars().map(parse_card).collect(), bid.parse().unwrap())
    }
}

mod part2 {
    use super::*;

    fn classify_hand(counts: &HashMap<u32, Vec<Card>>, jokers: u32) -> HandType {
        if jokers >= 4 || jokers == 3 && counts.contains_key(&2) {
            HandType::FiveOfAKind()
        } else if jokers == 3 {
            HandType::FourOfAKind()
        } else if jokers == 2 && counts.contains_key(&3) {
            HandType::FiveOfAKind()
        } else if jokers == 2 && counts.contains_key(&2) {
            HandType::FourOfAKind()
        } else if jokers == 2 {
            HandType::ThreeOfAKind()
        } else if jokers == 1 && counts.contains_key(&4) {
            HandType::FiveOfAKind()
        } else if jokers == 1 && counts.contains_key(&3) {
            HandType::FourOfAKind()
        } else if jokers == 1
            && let Some(two) = counts.get(&2)
            && two.len() == 2
        {
            HandType::FullHouse()
        } else if jokers == 1 && counts.contains_key(&2) {
            HandType::ThreeOfAKind()
        } else if jokers == 1 {
            HandType::OnePair()
        } else if counts.contains_key(&5) {
            HandType::FiveOfAKind()
        } else if counts.contains_key(&4) {
            HandType::FourOfAKind()
        } else if counts.contains_key(&3) && counts.contains_key(&2) {
            HandType::FullHouse()
        } else if counts.contains_key(&3) {
            HandType::ThreeOfAKind()
        } else if let Some(two) = counts.get(&2)
            && two.len() == 2
        {
            HandType::TwoPair()
        } else if counts.contains_key(&2) {
            HandType::OnePair()
        } else {
            HandType::HighCard()
        }
    }

    impl Hand {
        fn new_part2(cards: Vec<Card>, bid: u32) -> Self {
            let mut counts = count_cards(&cards);
            let jokers = counts.remove(&1).unwrap_or(0);
            let counts = reverse_counts(&counts);
            let hand_type = classify_hand(&counts, jokers);
            Self {
                cards,
                bid,
                hand_type,
            }
        }
    }

    fn parse_card(c: char) -> Card {
        match c {
            'J' => 1,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        }
    }

    pub fn parse_hand(line: &str) -> Hand {
        let (hand, bid) = line.split_once(' ').unwrap();
        Hand::new_part2(hand.chars().map(parse_card).collect(), bid.parse().unwrap())
    }
}

#[aoc_generator(day7, part1)]
fn input_generator(input: &str) -> Vec<Hand> {
    input.lines().map(part1::parse_hand).collect()
}

#[aoc_generator(day7, part2)]
fn input_generator_part2(input: &str) -> Vec<Hand> {
    input.lines().map(part2::parse_hand).collect()
}

#[aoc(day7, part1)]
fn part1(hands: &[Hand]) -> u32 {
    let mut hands = hands.to_owned();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(hands: &[Hand]) -> u32 {
    part1(hands)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(INPUT)), 6440);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator_part2(INPUT)), 5905);
    }
}
