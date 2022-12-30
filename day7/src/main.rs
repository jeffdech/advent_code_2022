mod fscmd;
use fscmd::*;

fn main() {
    let lines = parser::parse_input(include_str!("input.txt")).unwrap().1;
    let tree_root = walk_lines(lines);

    println!("{:?}", PrettyNode(&tree_root));
    let sum = all_dirs(tree_root.clone())
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();
    dbg!(sum);

    println!("Min space to remove = {}", min_removal(tree_root));
}