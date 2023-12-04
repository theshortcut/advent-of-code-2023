use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winners: Vec<u32>,
    candidates: Vec<u32>,
}

fn parse(input: &str) -> HashMap<u32, Card> {
    input
        .lines()
        .map(|l| {
            let (id, nums) = l.split_once(':').unwrap();
            let (_, id) = id.split_once(' ').unwrap();
            let id: u32 = id.trim().parse().unwrap();
            let (winners, candidates) = nums.split_once('|').unwrap();
            let winners = winners
                .trim()
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();
            let candidates = candidates
                .trim()
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();
            (
                id,
                Card {
                    winners,
                    candidates,
                },
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);
    let sum = cards
        .iter()
        .map(|(_, c)| {
            let winners = c
                .candidates
                .iter()
                .filter(|v| c.winners.contains(v))
                .count() as u32;
            match winners {
                0 => 0,
                1 => 1,
                i => (2 as u32).pow(i - 1),
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let mut cards_counts = HashMap::from(
        cards
            .keys()
            .map(|id| (*id, 1 as u32))
            .collect::<HashMap<u32, u32>>(),
    );
    for id in 1..=(cards.len() as u32) {
        let card = cards.get(&id).unwrap();
        let card_count = *cards_counts.get(&id).unwrap();
        let winners = card
            .candidates
            .iter()
            .filter(|v| card.winners.contains(v))
            .count() as u32;
        for id_delta in 1..=winners {
            if let Some(count) = cards_counts.get_mut(&(id + id_delta)) {
                *count = *count + card_count;
            }
        }
    }
    Some(cards_counts.iter().map(|(_, count)| count).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
