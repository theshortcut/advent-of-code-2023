use itertools::Itertools;

advent_of_code::solution!(14);

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn roll_north_and_weigh(platform: &[&[u8]]) -> u32 {
    let mut total_load = 0;
    for x in 0..platform[0].len() {
        let column = platform.iter().map(|c| c[x]).collect_vec();
        for (not_static, group) in &column.iter().enumerate().group_by(|(_, &c)| c != b'#') {
            if not_static {
                let group = group.collect_vec();
                let starting_index = group[0].0;
                let round_rock_count = group.iter().filter(|(_, &c)| c == b'O').count();
                for weight in (column.len() - round_rock_count + 1 - starting_index)
                    ..=(column.len() - starting_index)
                {
                    total_load += weight;
                }
            }
        }
    }
    total_load as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let platform = parse(input);
    let total = roll_north_and_weigh(&platform);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
