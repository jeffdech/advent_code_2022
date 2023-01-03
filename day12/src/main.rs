mod grid;
use grid::*;

fn main() {
    let grid = Grid::parse(include_str!("sample-input.txt"));
    println!("{grid:?}");
    println!("{:?}", grid.cell(&Coord::from((3, 2))));

    let mut walker = GraphWalker::new(&grid, grid.start(), grid.end());
    walker.search();
    
    let path = walker.path();
    println!("{:?}", path);
    println!("==== Part I ====");
    println!("The length is {}", path.len() - 1);
}
