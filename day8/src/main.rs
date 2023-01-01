mod grid;
use grid::*;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));

    println!("There are {} visible trees.", grid.visible_count());

    println!("(3, 2) scenic score is {}", grid.scenic_score(3, 2));
    println!("Max scenic score is {:?}", grid.max_scenic_score());
}
