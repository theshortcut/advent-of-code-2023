use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Add<&Dir> for &Point {
    type Output = Point;

    fn add(self, dir: &Dir) -> Self::Output {
        match dir {
            Dir::North => Point(self.0, self.1 - 1),
            Dir::West => Point(self.0 - 1, self.1),
            Dir::South => Point(self.0, self.1 + 1),
            Dir::East => Point(self.0 + 1, self.1),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn step(point: &Point, dir: &Dir, tiles: &Vec<Vec<u8>>) -> Vec<(Point, Dir)> {
    let width = tiles[0].len();
    let height = tiles.len();
    let should_continue = match (point, dir) {
        (Point(0, _), Dir::West) => false,
        (Point(x, _), Dir::East) if x == &(width - 1) => false,
        (Point(_, 0), Dir::North) => false,
        (Point(_, y), Dir::South) if y == &(height - 1) => false,
        _ => true,
    };
    if !should_continue {
        return vec![];
    }
    let next_coord = point + dir;
    let next_dirs = get_directions(&next_coord, dir, tiles);
    next_dirs.iter().map(|&d| (next_coord, d)).collect()
}

fn get_directions(point: &Point, dir: &Dir, tiles: &Vec<Vec<u8>>) -> Vec<Dir> {
    match (dir, tiles[point.1][point.0]) {
        (Dir::North, b'/') => vec![Dir::East],
        (Dir::South, b'/') => vec![Dir::West],
        (Dir::East, b'/') => vec![Dir::North],
        (Dir::West, b'/') => vec![Dir::South],
        (Dir::North, b'\\') => vec![Dir::West],
        (Dir::South, b'\\') => vec![Dir::East],
        (Dir::East, b'\\') => vec![Dir::South],
        (Dir::West, b'\\') => vec![Dir::North],
        (Dir::West | Dir::East, b'|') => vec![Dir::North, Dir::South],
        (Dir::North | Dir::South, b'-') => vec![Dir::West, Dir::East],
        (&dir, _) => vec![dir],
    }
}

fn trace_and_count(&point: &Point, dir: &Dir, tiles: &Vec<Vec<u8>>) -> u32 {
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    let mut beams = get_directions(&point, &dir, tiles)
        .iter()
        .map(|&dir| (point, dir))
        .collect_vec();
    while !beams.is_empty() {
        let (point, dir) = beams.pop().unwrap();
        if !seen.contains(&(point, dir)) {
            seen.insert((point, dir));
            energized.insert(point);
            let mut next = step(&point, &dir, tiles).iter().copied().collect_vec();
            beams.append(next.as_mut());
        }
    }
    energized.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse(input);
    Some(trace_and_count(&Point(0, 0), &Dir::East, &tiles))
}

pub fn part_two(input: &str) -> Option<u32> {
    let tiles = parse(input);
    let width = tiles[0].len();
    let height = tiles.len();
    let starting_beams = (0..height)
        .flat_map(|y| {
            (0..width)
                .flat_map(|x| match (x, y) {
                    (0, 0) => vec![(Point(x, y), Dir::East), (Point(x, y), Dir::South)],
                    (0, y) if y == height - 1 => {
                        vec![(Point(x, y), Dir::East), (Point(x, y), Dir::North)]
                    }
                    (x, y) if y == height - 1 && x == width - 1 => {
                        vec![(Point(x, y), Dir::West), (Point(x, y), Dir::North)]
                    }
                    (x, 0) if x == width - 1 => {
                        vec![(Point(x, y), Dir::West), (Point(x, y), Dir::South)]
                    }
                    (0, y) => vec![(Point(x, y), Dir::East)],
                    (x, y) if x == width - 1 => vec![(Point(x, y), Dir::West)],
                    (x, 0) => vec![(Point(x, y), Dir::South)],
                    (x, y) if y == height - 1 => vec![(Point(x, y), Dir::North)],
                    _ => vec![],
                })
                .collect_vec()
        })
        .collect_vec();
    starting_beams
        .iter()
        .map(|(point, dir)| trace_and_count(point, dir, &tiles))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
        assert_eq!(part_one("\\........\n/........"), Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
