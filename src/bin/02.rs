advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct Game {
    id: u32,
    dice_sets: Vec<DiceSet>,
}

#[derive(Debug, Default)]
struct DiceSet {
    blue: u32,
    green: u32,
    red: u32,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (game_id_str, sets_str) = line.split_once(':').unwrap();
            let (_, game_id_str) = game_id_str.split_once(' ').unwrap();
            let id = game_id_str.parse().unwrap();
            let dice_sets = sets_str
                .split(';')
                .map(|s| {
                    s.split(',').fold(DiceSet::default(), |mut dice_set, s| {
                        let (count_str, color) = s.trim().split_once(' ').unwrap();
                        let count = count_str.parse().unwrap();
                        match color {
                            "blue" => dice_set.blue = count,
                            "green" => dice_set.green = count,
                            "red" => dice_set.red = count,
                            _ => (),
                        }
                        dice_set
                    })
                })
                .collect();
            Game { id, dice_sets }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);
    let sum = games
        .iter()
        .filter(|g| {
            !g.dice_sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
        })
        .map(|g| g.id)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse(input);
    let sum = games
        .iter()
        .map(|g| {
            let red_count = g
                .dice_sets
                .iter()
                .max_by(|a, b| a.red.cmp(&b.red))
                .unwrap()
                .red;
            let green_count = g
                .dice_sets
                .iter()
                .max_by(|a, b| a.green.cmp(&b.green))
                .unwrap()
                .green;
            let blue_count = g
                .dice_sets
                .iter()
                .max_by(|a, b| a.blue.cmp(&b.blue))
                .unwrap()
                .blue;
            red_count * green_count * blue_count
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
