mod rucksack;

use crate::rucksack::*;

fn main() {
    let text = include_str!("input.txt");
    let sacks: Vec<Rucksack> = text
        .lines()
        .map(|l| Rucksack::parse(l))
        .collect();

    let sum_priority: usize = sacks.iter().map(|s| s.common_priority() as usize).sum();
    println!("{:?}", sum_priority);

    let elf_groups = parse_groups(text);
    let total_groups: usize = elf_groups.iter()
        .map(|g| g.common_priority() as usize)
        .sum();
    println!("The group total is {}", total_groups);
}
