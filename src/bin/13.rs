use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

advent_of_code::solution!(13);

// impl Iterator<Item = Vec<&[u8]>>
fn parse(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|s| s.as_bytes().to_vec()).collect())
        .collect()
}

fn find_reflection(pattern: &Vec<Vec<u8>>) -> Option<u32> {
    (1..pattern.len())
        .find(|&row| {
            let (initial, end) = pattern.split_at(row);
            initial.iter().rev().zip(end.iter()).all(|(a, b)| a == b)
        })
        .map(|r| r as u32)
}

fn transpose(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    let new = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .rev()
                .collect::<Vec<_>>()
        })
        .collect_vec();
    new
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .map(|pattern| match find_reflection(pattern) {
                Some(row) => 100 * row,
                None => {
                    let rotated = transpose(pattern.clone());
                    find_reflection(&rotated).unwrap()
                }
            })
            .sum(),
    )
}

fn find_reflection_with_smudge(pattern: &Vec<Vec<u8>>) -> Option<u32> {
    (1..pattern.len())
        .find(|&row| {
            let (initial, end) = pattern.split_at(row);
            initial
                .iter()
                .rev()
                .zip(end.iter())
                .fold_while(0, |acc, (a, b)| {
                    let diffs = a.iter().zip(b.iter()).filter(|(a, b)| a != b).count();
                    let acc = acc + diffs;
                    if acc < 2 {
                        Continue(acc)
                    } else {
                        Done(acc)
                    }
                })
                .into_inner()
                == 1
        })
        .map(|r| r as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .map(|pattern| match find_reflection_with_smudge(pattern) {
                Some(row) => 100 * row,
                None => {
                    let rotated = transpose(pattern.clone());
                    find_reflection_with_smudge(&rotated).unwrap()
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
