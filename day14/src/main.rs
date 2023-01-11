#![feature(generators)]
#![feature(iter_from_generator)]

mod sand;
use sand::*;

fn main() {
    let mut grid = Grid::parse(include_str!("input.txt"));
    println!("{grid:?}");

    loop {
        if let StepResult::Settled(Grid::SPAWN_POINT) = grid.step() {
            break;
        }
    }

    println!("Total sand: {}", grid.num_settled());
}
