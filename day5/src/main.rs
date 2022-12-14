mod crates;
use crates::parse::parse_all_crates;

use crate::crates::*;

fn main() {
    let text = include_str!("input.txt");
    let (_, (mut crates, instructions)) = parse::parse_input(text).unwrap();

    for ins in &instructions {
        println!("{:?}", ins);
    }
    
    println!("{:?}", crates);
    for inst in instructions {
        crates.apply(inst);
        println!("{:?}", crates);
    }

    println!("Answer 1 = {}", crates.top_crates());
}
