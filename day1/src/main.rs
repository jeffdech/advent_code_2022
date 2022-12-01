type ElfPacks = Vec<Vec<usize>>;

fn parse_text(text: &str) -> ElfPacks {
    let mut elves: ElfPacks = Vec::new();
    elves.push(Vec::new());

    for line in text.lines() {
        if line.is_empty() {
            elves.push(Vec::new());
        } else {
            let val: usize = line.parse().unwrap();
            elves.last_mut().unwrap().push(val);
        }
    }

    elves
}

fn max_calories(elves: &ElfPacks) -> usize {
    elves.iter()
        .map(|e| e.iter().sum())
        .max()
        .unwrap()
}

fn total_top_3(elves: &ElfPacks) -> usize {
    let mut totals = elves.iter()
        .map(|e| e.iter().sum())
        .collect::<Vec<usize>>();
    
    totals.sort();

    totals.iter()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let elves = parse_text(include_str!("input.txt"));
    
    println!("The max calories held is {}", max_calories(&elves));
    println!("The calories held by the top 3 are {}", total_top_3(&elves));
}