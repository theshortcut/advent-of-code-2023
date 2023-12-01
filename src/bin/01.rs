advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|l| {
            let first_digit: u32 = l.chars().find_map(|c| c.to_string().parse().ok()).unwrap();
            let last_digit: u32 = l
                .chars()
                .rev()
                .find_map(|c| c.to_string().parse().ok())
                .unwrap();
            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    Some(sum)
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|l| {
            let spelled_digits_with_position: Vec<(u32, u32)> = DIGITS
                .iter()
                .enumerate()
                .flat_map(|(digit, digit_string)| {
                    let digit = digit as u32;
                    l.match_indices(digit_string)
                        .map(move |(i, _)| (i as u32, digit))
                })
                .collect();
            let numerals_with_position: Vec<(u32, u32)> = l
                .chars()
                .enumerate()
                .filter_map(|(pos, c)| {
                    if let Some(digit) = c.to_string().parse().ok() {
                        return Some((pos as u32, digit));
                    }
                    None
                })
                .collect();
            let first_digit = spelled_digits_with_position
                .iter()
                .chain(numerals_with_position.iter())
                .min_by(|(pos, _), (pos2, _)| pos.cmp(pos2))
                .map(|(_pos, d)| d)
                .unwrap();
            let last_digit = spelled_digits_with_position
                .iter()
                .chain(numerals_with_position.iter())
                .max_by(|(pos, _), (pos2, _)| pos.cmp(pos2))
                .map(|(_pos, d)| d)
                .unwrap();
            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(369));
    }
}
