mod rps;

use rps::*;

fn main() {
    let moves = MoveList::parse(include_str!("input.txt"));

    let r1_score: usize = moves.moves.iter().map(|(s, o)| s.score(o)).sum();
    println!("Final score is {}", r1_score);
}
