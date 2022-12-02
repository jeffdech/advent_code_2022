use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Move::Rock => match other {
                Move::Rock => Ordering::Equal,
                Move::Paper => Ordering::Less,
                Move::Scissors => Ordering::Greater,
            },
            Move::Paper => match other {
                Move::Rock => Ordering::Greater,
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
            },
            Move::Scissors => match other {
                Move::Rock => Ordering::Less,
                Move::Paper => Ordering::Greater,
                Move::Scissors => Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Move {
    pub fn score(&self, other: &Self) -> usize {
        let shape_score = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let win_score = match self.cmp(other) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        shape_score + win_score
    }
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Win,
    Lose,
    Draw
}

impl RoundResult {
    pub fn matched_move(&self, matcher: &Move) -> Move {
        let opts = vec![Move::Rock, Move::Paper, Move::Scissors];

        *opts.iter().filter(|&opt| {
            match self {
                RoundResult::Win => opt > matcher,
                RoundResult::Lose => opt < matcher,
                RoundResult::Draw => opt == matcher,
            }
        })
        .next().unwrap()
    }
}

#[derive(Debug)]
pub struct MoveList {
    pub moves: Vec<(Move, Move)>,
}

impl MoveList {
    pub fn parse(text: &str) -> Self {
        let moves = text.lines()
            .map(|l| {
                let opp = match l.chars().nth(0).unwrap() {
                    'A' => Move::Rock,
                    'B' => Move::Paper,
                    'C' => Move::Scissors,
                    _ => unreachable!(),
                };

                let slf = match l.chars().nth(2).unwrap() {
                    'X' => Move::Rock,
                    'Y' => Move::Paper,
                    'Z' => Move::Scissors,
                    _ => unreachable!(),
                };

                (slf, opp)
            })
            .collect();

        Self {
            moves
        }
    }
}

#[derive(Debug)]
pub struct StratList {
    pub moves: Vec<(Move, RoundResult)>
}

impl StratList {
    pub fn parse(text: &str) -> Self {
        let moves = text.lines()
            .map(|l| {
                let opp = match l.chars().nth(0).unwrap() {
                    'A' => Move::Rock,
                    'B' => Move::Paper,
                    'C' => Move::Scissors,
                    _ => unreachable!(),
                };

                let slf = match l.chars().nth(2).unwrap() {
                    'X' => RoundResult::Lose,
                    'Y' => RoundResult::Draw,
                    'Z' => RoundResult::Win,
                    _ => unreachable!(),
                };

                (opp, slf)
            })
            .collect();

        Self {
            moves
        }
    }

    pub fn move_list(&self) -> MoveList {
        MoveList {
            moves: {
                self.moves.iter()
                    .map(|(opp, res)| (res.matched_move(opp), *opp))
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(Move::Rock < Move::Paper);
        assert!(Move::Rock > Move::Scissors);
        assert!(Move::Rock == Move::Rock);
        
        assert!(Move::Paper < Move::Scissors);
        assert!(Move::Paper > Move::Rock);
        assert!(Move::Paper == Move::Paper);

        assert!(Move::Scissors < Move::Rock);
        assert!(Move::Scissors > Move::Paper);
        assert!(Move::Scissors == Move::Scissors);
    }
}