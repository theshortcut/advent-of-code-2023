use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
enum Tile {
    PipeNS,
    PipeEW,
    PipeNE,
    PipeNW,
    PipeSE,
    PipeSW,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => Point(self.0, self.1 - 1),
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::PipeNS),
            '-' => Ok(Self::PipeEW),
            'L' => Ok(Self::PipeNE),
            'J' => Ok(Self::PipeNW),
            '7' => Ok(Self::PipeSW),
            'F' => Ok(Self::PipeSE),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err("Invalid character"),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.try_into().ok()).collect())
        .collect()
}

fn calculate_steps(tiles: &[Vec<Tile>]) -> usize {
    let start = tiles
        .iter()
        .find_position(|row| row.contains(&Tile::Start))
        .map(|(y, row)| {
            let (x, _) = row.iter().find_position(|&t| t == &Tile::Start).unwrap();
            Point(x, y)
        })
        .unwrap();
    let mut direction = if start.1 > 0
        && matches!(
            tiles[start.1 - 1][start.0],
            Tile::PipeSE | Tile::PipeSW | Tile::PipeNS
        ) {
        Direction::Up
    } else {
        Direction::Down
    };
    let mut position = start + direction;
    let mut steps = 1;
    loop {
        while matches!(tiles[position.1][position.0], Tile::PipeNS | Tile::PipeEW) {
            position = position + direction;
            steps += 1;
        }
        direction = match (&tiles[position.1][position.0], direction) {
            (Tile::PipeSW, Direction::Up) => Direction::Left,
            (Tile::PipeSE, Direction::Up) => Direction::Right,
            (Tile::PipeNW, Direction::Down) => Direction::Left,
            (Tile::PipeNE, Direction::Down) => Direction::Right,
            (Tile::PipeNW | Tile::PipeNE, _) => Direction::Up,
            (Tile::PipeSW | Tile::PipeSE, _) => Direction::Down,
            _ => {
                break;
            }
        };
        position = position + direction;
        steps += 1;
    }
    steps / 2
}


pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse(input);
    let steps = calculate_steps(&tiles);
    Some(steps as u32)
}

fn calculate_area(tiles: &[Vec<Tile>]) -> Option<u32> {
    let determinant = |a: Point, b: Point| (a.0 * b.1) as isize - (a.1 * b.0) as isize;

    let mut corner = tiles
        .iter()
        .find_position(|row| row.contains(&Tile::Start))
        .map(|(y, row)| {
            let (x, _) = row.iter().find_position(|&t| t == &Tile::Start).unwrap();
            Point(x, y)
        })
        .unwrap();
    let mut direction = if corner.1 > 0
        && matches!(
            tiles[corner.1 - 1][corner.0],
            Tile::PipeSE | Tile::PipeSW | Tile::PipeNS
        ) {
        Direction::Up
    } else {
        Direction::Down
    };
    let mut position = corner + direction;
    let mut steps = 1;
    let mut area = 0;
    loop {
        while matches!(tiles[position.1][position.0], Tile::PipeNS | Tile::PipeEW) {
            position = position + direction;
            steps += 1;
        }
        direction = match (&tiles[position.1][position.0], direction) {
            (Tile::PipeSW, Direction::Up) => Direction::Left,
            (Tile::PipeSE, Direction::Up) => Direction::Right,
            (Tile::PipeNW, Direction::Down) => Direction::Left,
            (Tile::PipeNE, Direction::Down) => Direction::Right,
            (Tile::PipeNW | Tile::PipeNE, _) => Direction::Up,
            (Tile::PipeSW | Tile::PipeSE, _) => Direction::Down,
            _ => {
                area += determinant(corner, position);
                break;
            }
        };
        area += determinant(corner, position);
        corner = position;
        position = position + direction;
        steps += 1;
    }
    (area.abs() / 2 - steps / 2 + 1).try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let tiles = parse(input);
    calculate_area(&tiles)
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(10));
    }
}
