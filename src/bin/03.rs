use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Point(u32, u32);

#[derive(Debug)]
struct PartNumber {
    start: Point,
    end: Point,
    val: u32,
}

fn parse_part_numbers(input: &str) -> Vec<PartNumber> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let re = Regex::new(r"\d+").unwrap();
            re.find_iter(line)
                .map(|m| PartNumber {
                    start: Point(
                        if m.start() == 0 {
                            m.start()
                        } else {
                            m.start() - 1
                        } as u32,
                        if y == 0 { y } else { y - 1 } as u32,
                    ),
                    end: Point(m.end() as u32, (y + 1) as u32),
                    val: m.as_str().parse().unwrap(),
                })
                .collect::<Vec<PartNumber>>()
        })
        .collect()
}

fn parse_symbols(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let re = Regex::new(r"[^0-9A-Za-z\.]").unwrap();
            re.find_iter(line)
                .map(|m| Point(m.start() as u32, y as u32))
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn parse_gears(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let re = Regex::new(r"\*").unwrap();
            re.find_iter(line)
                .map(|m| Point(m.start() as u32, y as u32))
                .collect::<Vec<Point>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let part_numbers = parse_part_numbers(input);
    let symbols = parse_symbols(input);
    let sum = part_numbers
        .iter()
        .filter(|p| {
            symbols.iter().any(|Point(x, y)| {
                x <= &p.end.0 && x >= &p.start.0 && y <= &p.end.1 && y >= &p.start.1
            })
        })
        .map(|p| p.val)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let part_numbers = parse_part_numbers(input);
    let gears = parse_gears(input);
    let sum = gears
        .iter()
        .map(|g| {
            let adjacent_parts = part_numbers
                .iter()
                .filter(|p| {
                    g.0 <= p.end.0 && g.0 >= p.start.0 && g.1 <= p.end.1 && g.1 >= p.start.1
                })
                .map(|p| p.val);
            if adjacent_parts.clone().count() == 2 {
                adjacent_parts.product()
            } else {
                0
            }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
