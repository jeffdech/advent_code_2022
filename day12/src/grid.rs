use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Start,
    End,
    Square(usize)
}

impl Cell {
    pub fn elevation(&self) -> usize {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            &Cell::Square(n) => n
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Cell {
        match value {
            'S' => Cell::Start,
            'E' => Cell::End,
            value if ('a'..='z').contains(&value) => Cell::Square(value as usize - 'a' as usize),
            _ => unreachable!()
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Square(n) => char::from(*n as u8 + 'a' as u8)
        };

        write!(f, "{c}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Coord {
        Coord {
            x: value.0,
            y: value.1
        }
    }
}

pub struct Grid {
    pub height: usize,
    pub width: usize,
    cells: Vec<Cell>
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        let cells: Vec<Cell> = input.lines()
            .map(|l| l.chars().map(|c| Cell::from(c)))
            .flatten()
            .collect();

        Self {
            height,
            width,
            cells
        }
    }

    pub fn idx2coord(&self, index: usize) -> Coord {
        let width = index % self.width;
        let height = (index - width) / self.width;
        Coord::from((width, height))
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        (coord.x < self.width) && (coord.y < self.height)
    }

    pub fn cell(&self, coord: &Coord) -> Option<&Cell> {
        if self.in_bounds(coord) {
            let idx = coord.y * self.width + coord.x;
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    pub fn walkable_neighbors<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + '_ {
        let curr_elevation = self.cell(coord).unwrap().elevation();
        let diffs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        diffs.into_iter()
            .filter_map(move |(dx, dy)| {
                Some(Coord {
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?
                })
            })
            .filter(|c| self.in_bounds(c))
            .filter(move |c| {
                let other_elevation = self.cell(c).unwrap().elevation();
                other_elevation <= curr_elevation + 1
            })
    }

    pub fn start(&self) -> Coord {
        self.cells.iter()
            .enumerate()
            .filter_map(|(n, &c)| {
                if c == Cell::Start {
                    Some(self.idx2coord(n))
                } else {
                    None
                }
            })
            .next()
            .unwrap()
    }

    pub fn end(&self) -> Coord {
        self.cells.iter()
            .enumerate()
            .filter_map(|(n, &c)| {
                if c == Cell::End {
                    Some(self.idx2coord(n))
                } else {
                    None
                }
            })
            .next()
            .unwrap()        
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} x {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let c = Coord::from((x, y));
                write!(f, "{:?}", self.cell(&c).unwrap());
            }
            writeln!(f);
        }

        Ok(())
    }
}

pub struct GraphWalker<'a> {
    grid: &'a Grid,
    start: Coord,
    end: Coord,
    distances: HashMap<Coord, usize>,
    previous: HashMap<Coord, Coord>,
    unvisited: HashSet<Coord>
}

impl<'a> GraphWalker<'a> {
    pub fn new(grid: &'a Grid, start: Coord, end: Coord) -> Self {
        let coords = (0..(grid.height * grid.width))
            .map(|n| grid.idx2coord(n))
            .collect::<Vec<Coord>>();

        let mut distances = coords.iter()
            .map(|&c| (c, usize::MAX))
            .collect::<HashMap<Coord, usize>>();
        distances.insert(start, 0);

        Self {
            grid,
            start,
            end,
            distances,
            previous: HashMap::new(),
            unvisited: coords.into_iter().collect()
        }
    }

    pub fn search(&mut self) {
        while !self.unvisited.is_empty() {
            let u = *self.unvisited.iter()
                .min_by_key(|&c| self.distances.get(c).unwrap())
                .unwrap();

            self.unvisited.remove(&u);

            for v in self.grid.walkable_neighbors(&u) {
                let alt = self.distances.get(&u).unwrap() + 1;
                if alt < *self.distances.get(&v).unwrap() {
                    self.previous.insert(v, u);
                    self.distances.insert(v, alt);
                }
            }
        }
    }

    pub fn path(&self) -> Vec<Coord> {
        let mut p = Vec::new();
        let mut u = self.end;

        p.push(u);

        while u != self.start {
            u = *self.previous.get(&u).unwrap();
            p.push(u);
        }

        p.into_iter().rev().collect()
    }
}