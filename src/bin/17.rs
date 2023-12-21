use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use advent_of_code::helpers::matrix::{Cell, Dir, Matrix};

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(Cell<u32>, Dir, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    position: Position,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(matrix: Matrix<u32>, minstep: isize, maxstep: isize) -> i64 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    while let Some((cost, (row, col, dist))) = queue.pop() {
        if (row, col) == (matrix.height - 1, matrix.width - 1) {
            return -cost;
        }
        if dists.get(&(row, col, dist)).is_some_and(|&c| -cost > c) {
            continue;
        }
        for (delta_y, delta_x) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dist == (delta_y, delta_x) || dist == (-delta_y, -delta_x) {
                continue;
            }
            let mut next_cost = -cost;
            for dist in 1..=maxstep {
                let cur_row = (row as isize + delta_y * dist) as usize;
                let cur_col = (col as isize + delta_x * dist) as usize;
                if cur_row >= matrix.height || cur_col >= matrix.width {
                    continue;
                }
                next_cost += matrix.get(cur_col, cur_row).unwrap() as i64;
                if dist < minstep {
                    continue;
                }
                let key = (cur_row, cur_col, (delta_y, delta_x));
                if next_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
                    dists.insert(key, next_cost);
                    queue.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Matrix<u32> = Matrix::from(input);
    let min_cost = dijkstra(matrix, 1, 3);
    min_cost.try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Matrix<u32> = Matrix::from(input);
    let min_cost = dijkstra(matrix, 4, 10);
    min_cost.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
