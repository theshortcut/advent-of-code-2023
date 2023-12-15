use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    fn dist(&self, other: &Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn parse(input: &str, expansion: usize) -> Vec<Point> {
    let mut expanded_rows = vec![];
    let mut expanded_columns = vec![];
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            if line.contains('#') {
                line.chars()
                    .enumerate()
                    .filter_map(|(column, char)| {
                        if row == 0 {
                            let column_str =
                                input.lines().filter_map(|l| l.chars().nth(column)).join("");
                            if !column_str.contains('#') {
                                expanded_columns.push(column);
                            }
                        }
                        let column_with_expansions = column
                            + expanded_columns.iter().filter(|&c| c < &column).count() * expansion;
                        match char {
                            '#' => Some(Point(
                                column_with_expansions,
                                row + expanded_rows.len() * expansion,
                            )),
                            _ => None,
                        }
                    })
                    .collect_vec()
            } else {
                expanded_rows.push(row);
                vec![]
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let tiles = parse(input, 1);
    let sum = tiles
        .iter()
        .tuple_combinations()
        .fold(0, |acc, (a, b)| acc + a.dist(b));
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let tiles = parse(input, 999999);
    let sum = tiles
        .iter()
        .tuple_combinations()
        .fold(0, |acc, (a, b)| acc + a.dist(b));
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
