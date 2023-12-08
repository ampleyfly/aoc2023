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

fn count_cards(cards: &Vec<Card>) -> HashMap<u32, Vec<Card>> {
    let mut counts = HashMap::new();
    for &card in cards {
        counts
            .entry(card)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }
    let mut result = HashMap::<u32, Vec<Card>>::new();
    for (card, count) in counts {
        result
            .entry(count)
            .and_modify(|vec| {
                vec.push(card);
            })
            .or_insert(vec![card]);
    }
    result
}

fn nth<T: std::fmt::Debug>(vec: &Vec<T>, n: usize) -> &T {
    vec.get(n).unwrap()
}

fn sort_desc<T: Ord>(vec: &mut Vec<T>) {
    vec.sort_by(|a, b| b.cmp(a));
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
        let result = match (cards1.get(0), cards2.get(0)) {
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

#[aoc_generator(day7, part1)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    input.lines().map(part1::parse_hand).collect()
}

#[aoc(day7, part1)]
pub fn part1(hands: &Vec<Hand>) -> u32 {
    let mut hands = hands.clone();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(&input_generator(input)), 6440);
    }
}
