use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split(' ').filter_map(|s| s.parse().ok()).collect())
        .collect()
}

fn get_history(report: &[i64]) -> Vec<Vec<i64>> {
    let mut history = vec![];
    history.push(report.to_vec());
    while !history.last().unwrap().iter().all(|&d| d == 0_i64) {
        let previous = history
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        history.push(previous);
    }
    history
}

fn extrapolate_next_value(report: &[i64]) -> i64 {
    let history = get_history(report);
    history
        .iter()
        .rev()
        .fold(0, |acc, h| acc + h.last().unwrap())
}

fn extrapolate_previous_value(report: &[i64]) -> i64 {
    let history = get_history(report);
    history
        .iter()
        .rev()
        .fold(0, |acc, h| h.first().unwrap() - acc)
}

pub fn part_one(input: &str) -> Option<i64> {
    let reports = parse(input);
    let sum = reports.iter().map(|r| extrapolate_next_value(r)).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let reports = parse(input);
    let sum = reports.iter().map(|r| extrapolate_previous_value(r)).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
