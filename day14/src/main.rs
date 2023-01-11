#![feature(generators)]
#![feature(iter_from_generator)]

mod sand;
use sand::*;

fn main() {
    let mut grid = Grid::parse(include_str!("input.txt"));
    println!("{grid:?}");

    loop {
        match grid.step() {
            StepResult::Moving(_) => {continue;},
            StepResult::Settled(_) => {println!("{grid:?}");}
            StepResult::Fell => {break;}
        }
    }

    println!("Total sand: {}", grid.num_settled());
}
