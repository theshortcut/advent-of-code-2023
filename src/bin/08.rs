use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Instruction>, HashMap<&str, (&str, &str)>) {
    let (instructions, node_map) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .chars()
        .filter_map(|c| match c {
            'R' => Some(Instruction::Right),
            'L' => Some(Instruction::Left),
            _ => None,
        })
        .collect::<Vec<_>>();
    let node_map = node_map
        .lines()
        .map(|l| {
            let (node, connections) = l.split_once(" = ").unwrap();
            let node = node.trim();
            let (left, right) = connections.trim().split_once(", ").unwrap();
            let left = &left[1..];
            let right = &right[..right.len() - 1];
            (node, (left, right))
        })
        .collect();
    (instructions, node_map)
}

fn navigate(
    starts: &[&str],
    ends: &[&str],
    instructions: &[Instruction],
    node_map: &HashMap<&str, (&str, &str)>,
) -> u64 {
    let steps = starts
        .iter()
        .map(|start| step(start, ends, instructions, node_map))
        .collect::<Vec<_>>();
    steps.iter().fold(steps[0], |acc, s| lcm(acc, *s))
}

fn step<'a>(
    start: &'a str,
    ends: &[&str],
    instructions: &[Instruction],
    node_map: &HashMap<&str, (&'a str, &'a str)>,
) -> u64 {
    let mut cur = start;
    let mut steps = 0;
    while !ends.contains(&cur) {
        let instruction = &instructions[steps % instructions.len()];
        let next = match instruction {
            Instruction::Left => node_map[cur].0,
            Instruction::Right => node_map[cur].1,
        };
        cur = next;
        steps += 1;
    }
    steps as u64
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let remainder = max % min;
        if remainder == 0 {
            return min;
        }

        max = min;
        min = remainder;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, node_map) = parse(input);
    let steps = navigate(&["AAA"], &["ZZZ"], &instructions, &node_map);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, node_map) = parse(input);
    let starts = node_map
        .clone()
        .into_keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();
    let ends = node_map
        .clone()
        .into_keys()
        .filter(|s| s.ends_with('Z'))
        .collect::<Vec<_>>();
    let steps = navigate(&starts, &ends, &instructions, &node_map);
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
