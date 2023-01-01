use std::collections::HashSet;
use std::fmt;
use std::iter::zip;
use nalgebra::base::{DMatrix, MatrixSlice};
use nalgebra::base::dimension::Dynamic;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {x, y}
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct TreeSet(HashSet<Coord>);

impl TreeSet {
    fn rows(&self) -> usize {
        self.0.iter()
            .map(|coord| coord.x)
            .max().unwrap() + 1
    }

    fn columns(&self) -> usize {
        self.0.iter()
            .map(|coord| coord.y)
            .max().unwrap() + 1
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }

    fn hit_map(&self) -> Vec<Vec<bool>> {
        let (rows, cols) = self.shape();

        let mut base: Vec<Vec<bool>> = (0..rows)
            .map(|_| vec![false; cols])
            .collect();

        for c in self.0.iter() {
            base[c.x][c.y] = true;
        }

        base
    }
}

impl fmt::Debug for TreeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.hit_map() {
            for val in row {
                if val {
                    write!(f, "x");
                } else {
                    write!(f, ".");
                }
            }
            writeln!(f);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Grid(DMatrix<i32>);

impl Grid {
    pub fn parse(i: &str) -> Self {
        let nrows: usize = i.lines().count();
        let mut lines = i.lines().peekable();

        let ncols: usize = lines.peek().expect("Should have first line").chars().count();
        let rows = i.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32))
            .flatten();
        Self(DMatrix::from_row_iterator(nrows, ncols, rows))
    }

    pub fn visibile_trees(&self) -> TreeSet {
        let max_coords = self.external_trees()
            .chain(self.row_visible(false))
            .chain(self.row_visible(true))
            .chain(self.col_visible(false))
            .chain(self.col_visible(true));

        // let max_coords = self.col_maxes(false);
        TreeSet(HashSet::<Coord>::from_iter(max_coords))
    }

    pub fn visible_count(&self) -> usize {
        self.visibile_trees().0.len()
    }

    fn external_trees(&self) -> impl Iterator<Item = Coord> + '_ {
        let (nrows, ncols) = self.0.shape();

        let top_row = (0..ncols).map(|c| Coord::new(0, c));
        let bottom_row = (0..ncols).map(|c| Coord::new(self.0.shape().0 - 1, c));
        let left_col = (1..nrows).map(|r| Coord::new(r, 0));
        let right_col = (1..nrows).map(|r| Coord::new(r, self.0.shape().0 - 1));

        top_row
            .chain(bottom_row)
            .chain(left_col)
            .chain(right_col)
            .into_iter()
    }

    fn row_visible(&self, reverse: bool) -> impl Iterator<Item = Coord> + '_ {
        let (nrows, ncols) = self.0.shape();
        (1..nrows-1)
            .map(move |row| {
                let mut visible = Vec::<Coord>::new();
                let mut max_val: i32 = -1;

                if !reverse {
                    zip((0..ncols-1), self.0.slice((row, 0), (1, ncols-1)).iter())
                        .for_each(|(col, &val)| {
                            if (val as i32) > max_val {
                                max_val = val as i32;
                                visible.push(Coord::new(row, col));
                            }
                        })
                } else {
                    zip((1..ncols), self.0.slice((row, 1), (1, ncols-1)).iter())
                        .rev()
                        .for_each(|(col, &val)| {
                            if (val as i32) > max_val {
                                max_val = val as i32;
                                visible.push(Coord::new(row, col));
                            }
                        })
                }

                visible.into_iter()
            })
            .flatten()
    }

    fn col_visible(&self, reverse: bool) -> impl Iterator<Item = Coord> + '_ {
        let (nrows, ncols) = self.0.shape();
        (1..ncols-1)
            .map(move |col| {
                let mut visible = Vec::<Coord>::new();
                let mut max_val = -1;

                if !reverse {
                    zip((0..nrows-1), self.0.slice((0, col), (nrows-1, 1)).iter())
                        .for_each(|(row, &val)| {
                            if (val as i32) > max_val {
                                max_val = val as i32;
                                visible.push(Coord::new(row, col));
                            }
                        })
                } else {
                    zip((1..nrows), self.0.slice((1, col), (nrows-1, 1)).iter())
                        .rev()
                        .for_each(|(row, &val)| {
                            if (val as i32) > max_val {
                                max_val = val as i32;
                                visible.push(Coord::new(row, col));
                            }
                        })
                }

                visible.into_iter()
            })
            .flatten()
    }

    pub fn scenic_score(&self, row: usize, col: usize) -> usize {
        let (nrows, ncols) = self.0.shape();
        let value = self.0[(row, col)];

        let slices = vec![
            (self.0.slice((0, col), (row, 1)), true),
            (self.0.slice((row + 1, col), (nrows - row - 1, 1)), false),
            (self.0.slice((row, 0), (1, col)), true),
            (self.0.slice((row, col + 1), (1, ncols - col - 1)), false)
        ];

        slices.iter()
            .map(|&(s, is_rev)| {
                let mut count = 0;
                if is_rev {
                    for &height in s.iter().rev() {
                        count += 1;
                        if height >= value {
                            break;
                        }
                    }
                } else {
                    for &height in &s {
                        count += 1;
                        if height >= value {
                            break;
                        }
                    }
                }
                count
            }).product()
    }

    pub fn max_scenic_score(&self) -> ((usize, usize), usize) {
        let (nrows, ncols) = self.0.shape();
        (0..nrows).map(move |row|{
            (0..ncols).map(move |col| ((row, col), self.scenic_score(row, col)))
        })
        .flatten()
        .max_by_key(|(_, val)| val.clone())
        .unwrap()
    }
}