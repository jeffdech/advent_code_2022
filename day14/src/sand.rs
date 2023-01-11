use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, derive_more::Add, derive_more::AddAssign, derive_more::Sub)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn parse(input: &str) -> Self {
        let mut tokens = input.split(",");
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap()
        }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum()
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Polyline {
    points: Vec<Point>
}

impl Polyline {
    pub fn parse(input: &str) -> Self {
        Self {
            points: input.split(" -> ").map(Point::parse).collect()
        }
    }

    pub fn path_points(&self) -> impl Iterator<Item = Point> + '_ {
        std::iter::from_generator(||{
            let mut points = self.points.iter().copied();
            let Some(mut a) = points.next() else { return; };
            yield a;

            loop {
                let Some(b) = points.next() else { return; };
                let delta = (b - a).signum();
                assert!((delta.x == 0) ^ (delta.y == 0));

                loop {
                    a += delta;
                    yield a;
                    if a == b {
                        break;
                    }
                }
            }
        })
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Air,
    Sand,
    Rock
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StepResult {
    Fell,
    Settled(Point),
    Moving(Point),
}

pub struct Grid {
    origin: Point,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    settled: usize,
    current: Point
}

impl Grid {
    pub const SPAWN_POINT: Point = Point { x: 500, y: 0 };
    pub fn parse(input: &str) -> Self {
        let polylines: Vec<_> = input.lines().map(Polyline::parse).collect();

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (200i32, i32::MAX, 800i32, i32::MIN);

        for point in polylines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(std::iter::once(&Grid::SPAWN_POINT))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        let origin = Point { x: min_x, y: min_y };
        let width: usize = (max_x - min_x + 1).try_into().unwrap();

        let height: usize = (max_y - min_y + 1).try_into().unwrap();
        let height = height + 2;

        dbg!("W: {}, H: {}", width, height);
        dbg!("Origin: {origin:?}");

        let mut grid = Self {
            origin,
            width,
            height,
            cells: vec![Cell::Air; width * height],
            settled: 0,
            current: Grid::SPAWN_POINT
        };

        for point in polylines.iter().flat_map(|p| p.path_points()) {
            *grid.cell_mut(point).unwrap() = Cell::Rock;
        }

        for x in min_x..max_x {
            let point = (x as i32, (height - 1) as i32).into();
            println!("{point:?}");
            *grid.cell_mut(point).unwrap() = Cell::Rock;
        }

        grid
    }

    fn cell_index(&self, point: Point) -> Option<usize> {
        let Point { x, y } = point - self.origin;
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn cell(&self, point: Point) -> Option<&Cell> {
        Some(&self.cells[self.cell_index(point)?])
    }

    fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
        let index = self.cell_index(point)?;
        Some(&mut self.cells[index])
    }

    pub fn num_settled(&self) -> usize {
        self.settled
    }

    pub fn current(&self) -> Point {
        self.current
    }

    pub fn step(&mut self) -> StepResult {
        let dirs: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];
        let posns = dirs.into_iter()
            .map(|d| self.current + d.into());

        for pos in posns {
            // we fell off the map
            if let None = self.cell_index(pos) {
                return StepResult::Fell;
            }

            // if we can go somewhere, do it
            if let Some(Cell::Air) = self.cell(pos) {
                self.current = pos;
                return StepResult::Moving(pos);
            }
        }

        // if we fell through to here, no options and stay here
        // and reset
        *self.cell_mut(self.current).unwrap() = Cell::Sand;
        let settle_pos = self.current;
        self.settled += 1;
        self.current = Grid::SPAWN_POINT;
        StepResult::Settled(settle_pos)
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _
                } + self.origin;

                let cell = self.cell(point).unwrap();
                let c = match cell {
                    Cell::Air => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o'
                };

                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}