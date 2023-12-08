use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardRank {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl CardRank {
    fn parse(value: char, joker: bool) -> Result<Self, &'static str> {
        match (value, joker) {
            ('2', _) => Ok(Self::Two),
            ('3', _) => Ok(Self::Three),
            ('4', _) => Ok(Self::Four),
            ('5', _) => Ok(Self::Five),
            ('6', _) => Ok(Self::Six),
            ('7', _) => Ok(Self::Seven),
            ('8', _) => Ok(Self::Eight),
            ('9', _) => Ok(Self::Nine),
            ('T', _) => Ok(Self::Ten),
            ('J', false) => Ok(Self::Jack),
            ('J', true) => Ok(Self::Joker),
            ('Q', _) => Ok(Self::Queen),
            ('K', _) => Ok(Self::King),
            ('A', _) => Ok(Self::Ace),
            _ => Err("Unexpected character"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct CardHand {
    cards: Vec<CardRank>,
}

impl CardHand {
    fn new(value: &str, joker: bool) -> Self {
        let cards = value
            .chars()
            .filter_map(|c| CardRank::parse(c, joker).ok())
            .collect();
        Self { cards }
    }

    fn value(&self) -> HandValue {
        let counts = self
            .cards
            .iter()
            .filter(|&c| c != &CardRank::Joker)
            .counts();
        let counts_values: Vec<_> = counts.values().collect();
        let joker_count = self.cards.iter().filter(|&c| c == &CardRank::Joker).count();
        let &&max_count = counts_values.iter().max().unwrap_or(&&0);
        if joker_count + max_count == 5 {
            HandValue::FiveOfAKind
        } else if joker_count + max_count == 4 {
            HandValue::FourOfAKind
        } else if (max_count == 3 && counts_values.contains(&&2))
            || joker_count > 2
            || (max_count == 3 && joker_count > 0)
            || (max_count == 2 && joker_count > 1)
            || (counts_values.iter().filter(|&c| c == &&2).count() == 2 && joker_count == 1)
        {
            HandValue::FullHouse
        } else if joker_count + max_count == 3 {
            HandValue::ThreeOfAKind
        } else if (counts_values.iter().filter(|&c| c == &&2).count() == 2)
            || (max_count == 2 && joker_count > 0)
            || joker_count > 1
        {
            HandValue::TwoPair
        } else if joker_count + max_count == 2 {
            HandValue::OnePair
        } else {
            HandValue::HighCard
        }
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value().cmp(&other.value()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

fn parse(input: &str, joker: bool) -> Vec<(CardHand, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = CardHand::new(hand, joker);
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rounds = parse(input, false);
    rounds.sort_by(|(a, _), (b, _)| a.cmp(b));
    let sum = rounds
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rounds = parse(input, true);
    rounds.sort_by(|(a, _), (b, _)| a.cmp(b));
    let sum = rounds
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
