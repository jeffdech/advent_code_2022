use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R'
        };

        write!(f, "{c}")
    }
}

#[derive(Copy, Clone)]
pub struct Instruction(Direction, usize);

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.0, self.1)
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| {
            let mut cs = l.chars();

            let dir = match cs.next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!()
            };

            cs.next();

            let inc = cs.next().unwrap().to_digit(10).unwrap() as usize;

            Instruction(dir, inc)
        })
        .collect()
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Default, Debug)]
pub struct Coord(isize, isize);

impl Coord {
    pub fn signum(&self) -> Self {
        Coord(self.0.signum(), self.1.signum())
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::convert::From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord(0, 1),
            Direction::Down => Coord(0, -1),
            Direction::Left => Coord(-1, 0),
            Direction::Right => Coord(1, 0),
        }
    }
}

pub struct HTState {
    pub head: Coord,
    pub tail: Coord,
    pub visited: HashSet<Coord>
}

impl Default for HTState {
    fn default() -> Self {
        let mut visited = HashSet::<Coord>::new();
        visited.insert(Coord::default());

        HTState {
            head: Coord::default(),
            tail: Coord::default(),
            visited
        }
    }
}

impl HTState {
    fn adjacent(&self) -> bool {
        let diff = self.head - self.tail;
        (diff.0.abs() <= 1) && (diff.1.abs() <= 1)
    }

    pub fn update(&mut self, inst: Instruction) {
        let dir = Coord::from(inst.0);
        let n = inst.1;

        for _ in 0..n {
            self.head = self.head + dir;

            if !self.adjacent() {
                let diff = self.head - self.tail;
                self.tail = self.tail + diff.signum();
                self.visited.insert(self.tail);
            }
        }
    }

    fn grid_bounds(&self) -> (Coord, Coord) {
        let htv = vec![self.head, self.tail];

        let xmin = self.visited.iter().chain(htv.iter()).map(|c| c.0).min().unwrap();
        let ymin = self.visited.iter().chain(htv.iter()).map(|c| c.1).min().unwrap();

        let xmax = self.visited.iter().chain(htv.iter()).map(|c| c.0).max().unwrap();
        let ymax = self.visited.iter().chain(htv.iter()).map(|c| c.1).max().unwrap();

        (Coord(xmin, ymin), Coord(xmax, ymax))
    }
}

impl fmt::Debug for HTState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (cmin, cmax) = self.grid_bounds();

        let nx = cmax.0 - cmin.0 + 1;
        let ny = cmax.1 - cmin.1 + 1;

        for y in (cmin.1..cmax.1+1).rev() {
            for x in (cmin.0..cmax.0+1) {
                let c = Coord(x, y);
                if c == Coord(0, 0) {
                    write!(f, "s");
                } else if c == self.head {
                    write!(f, "H");
                } else if c == self.tail {
                    write!(f, "T");
                } else if self.visited.contains(&c) {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
            writeln!(f);
        }

        Ok(())
    }
}