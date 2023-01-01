mod headtail;

use headtail::*;

fn main() {
    let instructions = parse_instructions(include_str!("input.txt"));
    let mut state = HTState::default();
    instructions.iter().for_each(|&ins| {
        state.update(ins);
        println!("== {:?} ==\n", ins);
        println!("{:?}", state);
    });

    println!("Visited locations = {}", state.visited.len());
}
