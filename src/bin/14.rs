use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::{repeat_n, Itertools};

advent_of_code::solution!(14);

enum Dir {
    North,
    West,
    South,
    East,
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn consolidate(line: &[u8], end: bool) -> Vec<u8> {
    let mut output = vec![];
    for (not_static, group) in &line.iter().enumerate().group_by(|(_, &c)| c != b'#') {
        if not_static {
            let group = group.collect_vec();
            let rock_count = group.iter().filter(|(_, &c)| c == b'O').count();
            let empty_count = group.len() - rock_count;
            let rocks = repeat_n(b'O', rock_count);
            let empty = repeat_n(b'.', empty_count);
            let combined = if end {
                empty.chain(rocks)
            } else {
                rocks.chain(empty)
            };
            output.append(combined.collect_vec().as_mut());
        } else {
            (0..group.count()).for_each(|_| {
                output.push(b'#');
            })
        }
    }
    output
}

fn roll(dir: &Dir, platform: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut output = vec![];
    for _ in 0..platform.len() {
        output.push(vec![b'.'; platform[0].len()]);
    }
    match dir {
        &Dir::North => {
            for x in 0..platform[0].len() {
                let column = platform.iter().map(|c| c[x]).collect_vec();
                consolidate(&column, false)
                    .iter()
                    .enumerate()
                    .for_each(|(y, &val)| {
                        output[y][x] = val;
                    });
            }
        }
        Dir::South => {
            for x in 0..platform[0].len() {
                let column = platform.iter().map(|c| c[x]).collect_vec();
                consolidate(&column, true)
                    .iter()
                    .enumerate()
                    .for_each(|(y, &val)| {
                        output[y][x] = val;
                    });
            }
        }
        Dir::West => {
            platform
                .iter()
                .enumerate()
                .for_each(|(y, row)| output[y] = consolidate(row, false));
        }
        Dir::East => {
            platform
                .iter()
                .enumerate()
                .for_each(|(y, row)| output[y] = consolidate(row, true));
        }
    }
    output
}

fn weigh(platform: &[Vec<u8>]) -> u32 {
    platform.iter().rev().enumerate().fold(0, |acc, (y, b)| {
        acc + b.iter().filter(|&b| b == &b'O').count() as u32 * (y as u32 + 1)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let platform = parse(input);
    let rolled = roll(&Dir::North, &platform);
    let total = weigh(&rolled);
    Some(total)
}

fn cycle(platform: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let start: Vec<Vec<u8>> = platform.to_owned();
    [Dir::North, Dir::West, Dir::South, Dir::East]
        .iter()
        .fold(start, |last, dir| roll(dir, &last))
}

pub fn part_two(input: &str) -> Option<u32> {
    let platform = parse(input);
    let mut cache: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let repeats = 1000000000;
    let result = (0..repeats)
        .fold_while(platform, |last, i| {
            if let Some(repeat_idx) = cache.get(&last) {
                let period = i - repeat_idx;
                let rem = (repeats - i) % period;
                Done(
                    cache
                        .iter()
                        .find(|(_, &i)| i == repeat_idx + rem)
                        .unwrap()
                        .0
                        .clone(),
                )
            } else {
                let next = cycle(&last);
                cache.insert(last, i);
                Continue(next)
            }
        })
        .into_inner();
    Some(weigh(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
