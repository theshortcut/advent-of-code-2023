use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
enum Op {
    LT(u32, char),
    GT(u32, char),
    Go,
}

#[derive(Debug, Clone)]
struct Command {
    op: Op,
    dest: String,
}

type Workflow = Vec<Command>;

fn parse(input: &str) -> (Vec<Part>, HashMap<String, Workflow>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows_regex = Regex::new(r"(?<name>.+)\{(?<rules>.+)+\}").unwrap();
    let workflows = workflows
        .lines()
        .map(|l| {
            let caps = workflows_regex.captures(l).unwrap();
            let name = caps["name"].to_string();
            (
                name,
                caps["rules"]
                    .split(',')
                    .map(|rule_str| match rule_str {
                        _ if rule_str.contains('<') => {
                            let (field, rest) = rule_str.split_once('<').unwrap();
                            let (amount, destination) = rest.split_once(':').unwrap();
                            Command {
                                op: Op::LT(amount.parse().unwrap(), field.chars().next().unwrap()),
                                dest: destination.into(),
                            }
                        }
                        _ if rule_str.contains('>') => {
                            let (field, rest) = rule_str.split_once('>').unwrap();
                            let (amount, destination) = rest.split_once(':').unwrap();
                            Command {
                                op: Op::GT(amount.parse().unwrap(), field.chars().next().unwrap()),
                                dest: destination.into(),
                            }
                        }
                        _ => Command {
                            op: Op::Go,
                            dest: rule_str.into(),
                        },
                    })
                    .collect(),
            )
        })
        .collect();
    let parts_regex = Regex::new(r"x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)}").unwrap();
    let parts = parts
        .lines()
        .map(|l| {
            let caps = parts_regex.captures(l).unwrap();
            Part {
                x: caps["x"].parse().unwrap(),
                m: caps["m"].parse().unwrap(),
                a: caps["a"].parse().unwrap(),
                s: caps["s"].parse().unwrap(),
            }
        })
        .collect();
    (parts, workflows)
}

fn execute(part: &Part, workflows: &HashMap<String, Workflow>, name: &String) -> bool {
    let commands = workflows.get(name).unwrap();
    let matching_command = commands
        .iter()
        .find(|c| match c {
            Command {
                op: Op::LT(amount, field),
                dest: _,
            } => match field {
                'x' => part.x < *amount,
                'm' => part.m < *amount,
                'a' => part.a < *amount,
                's' => part.s < *amount,
                _ => unreachable!(),
            },
            Command {
                op: Op::GT(amount, field),
                dest: _,
            } => match field {
                'x' => part.x > *amount,
                'm' => part.m > *amount,
                'a' => part.a > *amount,
                's' => part.s > *amount,
                _ => unreachable!(),
            },
            Command {
                op: Op::Go,
                dest: _,
            } => true,
        })
        .unwrap();
    match matching_command.dest.as_str() {
        "A" => true,
        "R" => false,
        s => execute(part, workflows, &s.to_string()),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (parts, workflows) = parse(input);
    let start = "in".to_string();
    let sum = parts
        .iter()
        .filter(|p| execute(p, &workflows, &start))
        .fold(0, |acc, p| acc + p.x + p.m + p.a + p.s);
    Some(sum)
}

fn count_accepted(
    workflows: &HashMap<String, Vec<Command>>,
    curr: &str,
    mut ranges: [Vec<usize>; 4],
) -> usize {
    if curr == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if curr == "R" {
        return 0;
    }
    let mut ans = 0;
    let commands = &workflows[curr];
    for command in commands {
        match command {
            Command {
                op: Op::LT(amount, field),
                dest,
            } => {
                let i = "xmas".chars().position(|c| c == *field).unwrap();
                let mut newranges = ranges.clone();
                (newranges[i], ranges[i]) =
                    ranges[i].iter().partition(|&&val| val < *amount as usize);
                ans += count_accepted(workflows, dest.as_str(), newranges);
            }
            Command {
                op: Op::GT(amount, field),
                dest,
            } => {
                let i = "xmas".chars().position(|c| c == *field).unwrap();
                let mut newranges = ranges.clone();
                (newranges[i], ranges[i]) =
                    ranges[i].iter().partition(|&&val| val > *amount as usize);
                ans += count_accepted(workflows, dest.as_str(), newranges);
            }
            Command { op: Op::Go, dest } => {
                ans += count_accepted(workflows, dest.as_str(), ranges.clone());
            }
        };
    }
    ans
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, workflows) = parse(input);
    let start = "in";
    let starting_range: Vec<usize> = (1..=4000).collect();
    let mut ranges = [vec![], vec![], vec![], vec![]];
    ranges.fill(starting_range);
    Some(count_accepted(&workflows, start, ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
