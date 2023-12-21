use advent_of_code::helpers::matrix::{Dir, Point};

advent_of_code::solution!(18);

#[derive(Debug)]
struct Step {
    dir: Dir,
    len: isize,
}

fn parse_one(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|line| {
            let (dir, rest) = line.split_once(' ').unwrap();
            let (len, _) = rest.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Dir::N,
                "L" => Dir::W,
                "D" => Dir::S,
                "R" => Dir::E,
                _ => unreachable!(),
            };
            let len = len.parse().unwrap();
            Step { dir, len }
        })
        .collect()
}

fn execute_plan(plan: &[Step]) -> isize {
    let mut current = Point { x: 0_isize, y: 0 };
    let mut border = 0;
    let mut shoelace = (0, 0);

    plan.iter().for_each(|step| {
        let next = current.moved(&step.dir, &step.len);
        border += step.len;
        shoelace.0 += current.x * next.y;
        shoelace.1 += current.y * next.x;
        current = next;
    });

    let area = (shoelace.0.abs_diff(shoelace.1) / 2) as isize;
    let interior = 1 + area - border / 2;
    interior + border
}

pub fn part_one(input: &str) -> Option<isize> {
    let plan = parse_one(input);
    let size = execute_plan(&plan);
    Some(size)
}

fn parse_two(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(' ').unwrap();
            let (_, hex) = rest.split_once(' ').unwrap();
            let len = isize::from_str_radix(&hex[2..7], 16).unwrap();
            let dir = match &hex[7..hex.len()-1] {
                "0" => Dir::E,
                "1" => Dir::S,
                "2" => Dir::W,
                "3" => Dir::N,
                _ => unreachable!(),
            };
            Step { dir, len }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<isize> {
    let plan = parse_two(input);
    let size = execute_plan(&plan);
    Some(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
