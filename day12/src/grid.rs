use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Start,
    End,
    Square(usize)
}

impl Cell {
    fn elevation(&self) -> usize {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(n) => *n
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Square(n) => {
                char::from(*n as u8 + 'a' as u8)
            }
        };

        write!(f, "{c}")
    }
}

impl TryFrom<char> for Cell {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            value if ('a'..='z').contains(&value) => {
                let val = value as u8 - 'a' as u8;
                Ok(Cell::Square(val as usize))
            },
            _ => Err("Could not parse, not valid character")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        let cells = input.lines()
            .map(|l| {
                l.chars()
                    .map(|c| Cell::try_from(c).unwrap())
            })
            .flatten()
            .collect::<Vec<Cell>>();

        Grid {
            width,
            height,
            cells
        }
    }

    fn in_bounds(&self, coord: Coord) -> bool {
        (coord.x < self.width) && (coord.y < self.height)
    }

    fn cell(&self, coord: Coord) -> Option<&Cell> {
        if self.in_bounds(coord) {
            let idx = coord.y * self.width + coord.x;
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    fn cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        if self.in_bounds(coord) {
            let idx = coord.y * self.width + coord.x;
            Some(&mut self.cells[idx])
        } else {
            None
        }       
    }

    pub fn walkable_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let elevation = self.cell(coord).unwrap().elevation();
        let diffs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        diffs.into_iter()
            .filter_map(move |(dx, dy)| {
                Some(Coord{
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?,
                })
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(move |&coord| {
                let other_elevation = self.cell(coord).unwrap().elevation();
                other_elevation <= elevation + 1
            })
    }

    pub fn idx2coord(&self, idx: usize) -> Coord {
        let x = idx % self.width;
        let y = (idx - x) / self.width;
        (x, y).into()
    }

    pub fn start(&self) -> Coord {
        let sidx = self.cells.iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if *v == Cell::Start {
                    Some(idx)
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        
        self.idx2coord(sidx)
    }

    pub fn end(&self) -> Coord {
        let eidx = self.cells.iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if *v == Cell::End {
                    Some(idx)
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        
        self.idx2coord(eidx)        
    }

    pub fn at_height(&self, height: usize) -> Vec<Coord> {
        self.cells.iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if c.elevation() == height {
                    Some(idx)
                } else {
                    None
                }
            })
            .map(|idx| self.idx2coord(idx))
            .collect()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.cell((x, y).into()).unwrap();
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct GraphRunner<'a> {
    grid: &'a Grid,
    distances: HashMap<Coord, usize>,
    previous: HashMap<Coord, Option<Coord>>,
    unvisited: HashSet<Coord>,
    target: Coord,
    start: Coord,
}

impl<'a> GraphRunner<'a> {
    pub fn new(grid: &'a Grid, start: Coord) -> Self {
        let coords = (0..(grid.height * grid.width))
            .map(|n| grid.idx2coord(n))
            .collect::<Vec<_>>();

        let mut distances: HashMap<Coord, usize> = coords.iter()
            .map(|&c| (c, usize::MAX))
            .collect();

        distances.insert(grid.start(), 0);

        let unvisited: HashSet<Coord> = coords.iter().cloned().collect();

        let previous: HashMap<Coord, Option<Coord>> = coords.iter()
            .map(|&c| (c, None))
            .collect();

        Self {
            grid,
            distances,
            previous,
            unvisited,
            target: grid.end(),
            start,
        }
    }

    pub fn find_path(&mut self) -> Option<Vec<Coord>> {
        while !self.unvisited.is_empty() {
            let u = *self.unvisited.iter()
                .min_by_key(|c| self.distances.get(c).unwrap())
                .unwrap();
            
            if u == self.target {
                break;
            }

            self.unvisited.remove(&u);

            let next_points = self.grid.walkable_neighbors(u).filter(|c| self.unvisited.contains(&c));
            for v in next_points {
                let alt = *self.distances.get(&u).unwrap() + 1;
                if alt < *self.distances.get(&v).unwrap() {
                    self.distances.insert(v, alt);
                    self.previous.insert(v, Some(u));
                }
            }
        }

        self.path()
    }

    fn path(&self) -> Option<Vec<Coord>> {
        let mut p = Vec::new();
        let mut u = Some(self.target);

        if let None = self.previous.get(&u.unwrap()).unwrap() {
            return None;
        }

        while (u != Some(self.start)) && (u != None) {
            p.push(u.unwrap());
            u = *self.previous.get(&u.unwrap()).unwrap();
        }

        if p.is_empty() {
            return None;
        }

        Some(p.into_iter().rev().collect())
    }

    fn dump_grid(&self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let coord = Coord::from((x, y));

            }
        }
    }
}