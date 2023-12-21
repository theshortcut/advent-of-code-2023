use std::{
    convert::From,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Dir {
    pub fn invert(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::NW => Dir::SE,
            Dir::NE => Dir::SW,
            Dir::SE => Dir::NW,
            Dir::SW => Dir::NE,
        }
    }
}

pub static CARDINALS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

pub static ORDINALS: [Dir; 4] = [Dir::NW, Dir::NE, Dir::SE, Dir::SW];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: Clone = usize> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Clone + Add<Output = T> + Sub<Output = T>> Point<T> {
    pub fn moved(&self, dir: &Dir, len: &T) -> Point<T> {
        let Point { x, y } = self;
        match dir {
            Dir::N => Point {
                x: *x,
                y: *y - *len,
            },
            Dir::E => Point {
                x: *x + *len,
                y: *y,
            },
            Dir::S => Point {
                x: *x,
                y: *y + *len,
            },
            Dir::W => Point {
                x: *x - *len,
                y: *y,
            },
            Dir::NE => Point {
                x: *x + *len,
                y: *y - *len,
            },
            Dir::SE => Point {
                x: *x + *len,
                y: *y + *len,
            },
            Dir::SW => Point {
                x: *x - *len,
                y: *y + *len,
            },
            Dir::NW => Point {
                x: *x - *len,
                y: *y - *len,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T: Copy + Display = char> {
    pub val: T,
    pub point: Point,
}

impl<T: Copy + Display> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl<T: Copy + Display> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl<T: Copy + Display> Display for Cell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}] ({})", self.point.x, self.point.y, self.val)
    }
}

#[derive(Debug)]
pub struct Matrix<T: Copy = char> {
    pub cells: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl From<&str> for Matrix<char> {
    fn from(s: &str) -> Self {
        let cells: Vec<Vec<char>> = s
            .lines()
            .filter_map(|l| {
                if !l.is_empty() {
                    Some(l.chars().collect())
                } else {
                    None
                }
            })
            .collect();

        Self {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }
}

impl From<&str> for Matrix<u32> {
    fn from(s: &str) -> Self {
        let cells: Vec<Vec<u32>> = s
            .lines()
            .filter_map(|l| {
                if !l.is_empty() {
                    Some(l.chars().map(|x| x.to_digit(10).unwrap()).collect())
                } else {
                    None
                }
            })
            .collect();

        Self {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }
}

impl<T: Copy + Display> Matrix<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.cells.get(y).and_then(|l| l.get(x).copied())
    }

    pub fn get_row(&self, y: usize) -> Option<&Vec<T>> {
        self.cells.get(y)
    }

    pub fn get_row_mut(&mut self, y: usize) -> Option<&mut Vec<T>> {
        self.cells.get_mut(y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.cells.get_mut(y).and_then(|l| l.get_mut(x))
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell<T>> {
        self.get(x, y).map(|val| Cell {
            point: Point { x, y },
            val,
        })
    }

    pub fn items(&self) -> impl Iterator<Item = Cell<T>> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.get_cell(x, y).unwrap())
    }

    pub fn neighbor(&self, cell: &Cell<T>, dir: &Dir) -> Option<Cell<T>> {
        match dir {
            Dir::NW => {
                let y = cell.point.y.checked_sub(1)?;
                let x = cell.point.x.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::N => {
                let x = cell.point.x;
                let y = cell.point.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::NE => {
                let x = cell.point.x + 1;
                let y = cell.point.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::W => {
                let x = cell.point.x.checked_sub(1)?;
                let y = cell.point.y;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::E => {
                let x = cell.point.x + 1;
                let y = cell.point.y;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::SW => {
                let x = cell.point.x.checked_sub(1)?;
                let y = cell.point.y + 1;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::S => {
                let y = cell.point.y + 1;
                let x = cell.point.x;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
            Dir::SE => {
                let x = cell.point.x + 1;
                let y = cell.point.y + 1;
                let val = self.get(x, y)?;
                Some(Cell {
                    point: Point { x, y },
                    val,
                })
            }
        }
    }

    pub fn neighbors<'a, 'b: 'a>(
        &'a self,
        start: &'b Cell<T>,
        directions: &'b [Dir],
    ) -> impl Iterator<Item = (Dir, Option<Cell<T>>)> + '_ {
        directions.iter().map(move |dir| {
            let neighbor = self.neighbor(start, dir);
            (*dir, neighbor)
        })
    }

    pub fn all_neighbors(
        &self,
        start: Cell<T>,
        include_ordinals: bool,
    ) -> impl Iterator<Item = (Dir, Option<Cell<T>>)> + '_ {
        let mut neighbors = Vec::from(CARDINALS);

        if include_ordinals {
            neighbors.extend(Vec::from(ORDINALS));
        }

        neighbors.into_iter().map(move |dir| {
            let neighbor = self.neighbor(&start, &dir);
            (dir, neighbor)
        })
    }

    pub fn area(
        &self,
        x_start: usize,
        x_end: usize,
        y_start: usize,
        y_end: usize,
    ) -> impl Iterator<Item = Cell<T>> + '_ {
        (x_start..=x_end)
            .cartesian_product(y_start..=y_end)
            .filter_map(|(x, y)| {
                self.get(x, y).map(|val| Cell {
                    point: Point { x, y },
                    val,
                })
            })
    }

    pub fn transpose(&mut self) {
        let mut res = vec![vec![]; self.height];

        self.cells.iter().for_each(|row| {
            for (i, val) in row.iter().enumerate() {
                res[i].push(*val);
            }
        });

        self.cells = res;
    }

    pub fn rotate_clockwise(&mut self) {
        self.transpose();
        self.cells.iter_mut().for_each(|row| {
            row.reverse();
        })
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.cells.iter_mut().for_each(|row| {
            row.reverse();
        });

        self.transpose();
    }
}
