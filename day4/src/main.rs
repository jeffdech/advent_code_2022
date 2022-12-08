mod seats;

use crate::seats::*;

fn main() {
    let text = include_str!("input.txt");
    let range_groups = text.lines()
        .map(|l| RangePair::parse(l))
        .collect::<Vec<_>>();

    range_groups.iter()
        .for_each(|rg| println!("{} : {}", rg, rg.overlaps()));

    println!("{} pairs have one containing the other", range_groups.iter().filter(|rg| rg.has_contained()).count());
    println!("{} pairs overlap", range_groups.iter().filter(|rg| rg.overlaps()).count());
}
