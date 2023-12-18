use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Condition {
    Damaged,
    Working,
    Unknown,
}

fn parse(input: &str) -> Vec<(Vec<Condition>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (field_str, spans_str) = line.split_once(' ').unwrap();
            let springs = field_str
                .chars()
                .map(|char| match char {
                    '.' => Condition::Working,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => unreachable!("Unknown character"),
                })
                .collect();
            let spans = spans_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            (springs, spans)
        })
        .collect()
}

fn possibilities(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    conditions: &[Condition],
    within: Option<usize>,
    remaining: &[usize],
) -> usize {
    if conditions.is_empty() {
        match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        }
    } else if within.is_some() && remaining.is_empty() {
        0
    } else {
        let key = (conditions.len(), within.unwrap_or(0), remaining.len());
        if let Some(&x) = cache.get(&key) {
            x
        } else {
            let ways = match (conditions[0], within) {
                (Condition::Working, Some(x)) if x != remaining[0] => 0,
                (Condition::Working, Some(_)) => {
                    possibilities(cache, &conditions[1..], None, &remaining[1..])
                }
                (Condition::Working, None) => {
                    possibilities(cache, &conditions[1..], None, remaining)
                }
                (Condition::Damaged, Some(_)) => {
                    possibilities(cache, &conditions[1..], within.map(|x| x + 1), remaining)
                }
                (Condition::Damaged, None) => {
                    possibilities(cache, &conditions[1..], Some(1), remaining)
                }
                (Condition::Unknown, Some(x)) => {
                    let mut ans =
                        possibilities(cache, &conditions[1..], within.map(|x| x + 1), remaining);
                    if x == remaining[0] {
                        ans += possibilities(cache, &conditions[1..], None, &remaining[1..])
                    }
                    ans
                }
                (Condition::Unknown, None) => {
                    possibilities(cache, &conditions[1..], Some(1), remaining)
                        + possibilities(cache, &conditions[1..], None, remaining)
                }
            };
            cache.insert(key, ways);
            ways
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let sum = parse(input)
        .iter()
        .map(|(conditions, spans)| {
            cache.clear();
            possibilities(&mut cache, conditions, None, spans)
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let sum = parse(input)
        .iter()
        .map(|(conditions, spans)| {
            let condition_join = vec![Condition::Unknown];
            let repeated_conditions =
                itertools::intersperse((0..5).map(|_| conditions.as_slice()), &condition_join)
                    .flatten()
                    .copied()
                    .collect_vec();
            let repeated_spans = spans.repeat(5);
            cache.clear();
            possibilities(
                &mut cache,
                repeated_conditions.as_slice(),
                None,
                &repeated_spans,
            )
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
