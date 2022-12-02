mod rps;

use rps::*;

fn main() {
    let moves = MoveList::parse(include_str!("input.txt"));

    let r1_score: usize = moves.moves.iter().map(|(s, o)| s.score(o)).sum();
    println!("Final score is {}", r1_score);

    let needed_moves = StratList::parse(include_str!("input.txt")).move_list();
    let r2_score: usize = needed_moves.moves.iter().map(|(s, o)| s.score(o)).sum();
    println!("Playing correctly, you'll get {}", r2_score);
}
