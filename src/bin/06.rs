advent_of_code::solution!(6);

fn calculate_winning_options(&time: &u64, &record: &u64) -> Vec<u64> {
    let mut options = vec![];
    for button_press_time in 1..time {
        let travel_time = time - button_press_time;
        if button_press_time * travel_time > record {
            options.push(button_press_time);
        }
    }
    options
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    let times = times
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();
    let distances = distances
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();
    let product = times
        .iter()
        .enumerate()
        .map(|(idx, time)| {
            let distance = distances[idx];
            calculate_winning_options(time, &distance).len() as u64
        })
        .product();
    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let record = lines.next().unwrap();
    let time: u64 = time
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    let record: u64 = record
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    let options = calculate_winning_options(&time, &record);
    Some(options.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
