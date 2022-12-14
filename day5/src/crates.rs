use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{u16, digit1, newline, space1},
    combinator::{all_consuming, map, opt},
    multi::{separated_list1, many1},
    sequence::{delimited, preceded, tuple, terminated},
    Finish, IResult,
};

use itertools::Itertools;

pub struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize
}

pub struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {i}: {:?}", pile)?;
        }
        Ok(())
    }
}

impl Piles {
    pub fn apply(&mut self, inst: Instruction) {
        for _ in 0..inst.quantity {
            let elem = self.0[inst.source].pop().unwrap();
            self.0[inst.destination].push(elem);
        }
    }

    pub fn top_crates(&self) -> String {
        self.0.iter().map(|c| c.last().unwrap().0).join("")
    }
}

fn transpose_rev_filter<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub mod parse {
    use super::*;

    fn parse_crate(i: &str) -> IResult<&str, Crate> {
        let first_char = |s: &str| Crate(s.chars().next().unwrap());
        let f = delimited(tag("["), take(1_usize), tag("]"));
        map(f, first_char)(i)
    }

    fn parse_hole(i: &str) -> IResult<&str, ()> {
        map(tag("   "), drop)(i)
    }

    fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
        alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
    }

    fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
        let (mut i, c) = parse_crate_or_hole(i)?;
        let mut v = vec![c];

        loop {
            let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
            match maybe_c {
                Some(c) => v.push(c),
                None => break,
            }
            i = next_i;
        }

        Ok((i, v))
    }

    pub fn parse_all_crates(i: &str) -> IResult<&str, Vec<Vec<Crate>>> {
        let (rest, crate_lines) = many1(terminated(parse_crate_line, newline))(i)?;
        let crate_cols = transpose_rev_filter(crate_lines);
        Ok((rest, crate_cols))
    }

    fn parse_number_line(i: &str) -> IResult<&str, ()> {
        let (rest, _) = terminated(many1(alt((digit1, space1))), newline)(i)?;
        Ok((rest, ()))
    }

    fn index_number(i: &str) -> IResult<&str, usize> {
        map(u16, |n| (n - 1) as usize)(i)
    }

    fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                preceded(tag("move "), map(u16, |n| n as usize)),
                preceded(tag(" from "), index_number),
                preceded(tag(" to "), index_number)
            )),
            |(q, s, d)| Instruction {
                quantity: q,
                source: s,
                destination: d
            }
        )(i)
    }

    pub fn parse_input(i: &str) -> IResult<&str, (Piles, Vec<Instruction>)> {
        let (rest, (crt, _, _, inst, _)) = tuple((
                map(parse_all_crates, |cs| Piles(cs)),
                parse_number_line,
                newline,
                separated_list1(newline, parse_instruction),
                opt(newline),
        ))(i)?;

        Ok((rest, (crt, inst)))
    }

    #[cfg(test)]
    mod test_parser {
        use super::*;

        #[test]
        fn test_parse_number_line() {
            assert_eq!(parse_number_line(" 1   2   3 \n"), Ok(("", ())));
        }

        #[test]
        fn test_parse_instruction() {
            let exp_ins = Instruction {
                quantity: 1,
                source: 1,
                destination: 0,
            };

            assert_eq!(parse_instruction("move 1 from 2 to 1"), Ok(("", exp_ins)));
        }
    }
}