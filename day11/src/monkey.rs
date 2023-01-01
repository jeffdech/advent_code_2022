use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{space1, newline},
    combinator::{opt, map},
    multi::{separated_list1, many1, many0},
    sequence::{delimited, preceded, tuple, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub enum Term {
    Old,
    Const(i64)
}

impl Term {
    fn eval(&self, old: i64) -> i64 {
        match *self {
            Term::Old => old,
            Term::Const(n) => n,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Add(Term, Term),
    Mult(Term, Term)
}

impl Op {
    pub fn eval(&self, old: i64) -> i64 {
        match self {
            Op::Add(t1, t2) => t1.eval(old) + t2.eval(old),
            Op::Mult(t1, t2) => t1.eval(old) * t2.eval(old),
        }
    }
}

#[derive(Debug)]
pub struct WorryTest {
    divisor: i64,
    when_true: i64,
    when_false: i64
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: WorryTest,
    inspections: usize,
}

impl Monkey {
    fn next(&mut self) -> Option<(usize, i64)> {
        let mut worry = self.items.pop_front()?;
        worry = self.op.eval(worry) / 3;

        let monkey_idx = if (worry % self.test.divisor) == 0 {
            self.test.when_true
        } else {
            self.test.when_false
        };

        self.inspections += 1;

        Some((monkey_idx as usize, worry))
    }

    fn process_items(&mut self) -> Vec<(usize, i64)> {
        let mut items = Vec::new();
        while let Some(item) = self.next() {
            items.push(item);
        }

        items
    }
}

pub struct MonkeyGroup {
    rounds: u64,
    monkeys: Vec<Monkey>,
}

impl MonkeyGroup {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        MonkeyGroup {
            rounds: 0,
            monkeys
        }
    }

    pub fn parse(input: &str) -> Self {
        MonkeyGroup::new(parse_input(input))
    }

    pub fn step_round(&mut self) {
        let nmonkeys = self.monkeys.len();
        for n in 0..nmonkeys {
            let items = self.monkeys[n].process_items();
            for (idx, worry) in items {
                self.monkeys[idx].items.push_back(worry);
            }
        }

        self.rounds += 1;
    }

    pub fn display_inspections(&self) {
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {} inspected {} items", idx, monkey.inspections);
        }
    }

    pub fn monkey_business(&self) -> usize {
        let mut counts = self.monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
        counts.sort_by_key(|&k| -1 * (k as isize));

        println!("{:?}", counts);

        counts[0..2].iter().product()
    }
}

impl fmt::Debug for MonkeyGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "After round {}", self.rounds);
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            write!(f, "Monkey {idx}: ");

            let nstrings: Vec<_> = monkey.items
                .iter()
                .map(|i| i.to_string())
                .collect();

            writeln!(f, "{}", nstrings.join(", "));
        }

        Ok(())
    }
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    many1(terminated(parser::parse_monkey, many0(newline)))(input).unwrap().1
}

mod parser {
    use super::*;

    fn parse_items(i: &str) -> IResult<&str, Vec<i64>> {
        delimited(
            tuple((space1, tag("Starting items: "))),
            separated_list1(tag(", "), nom::character::complete::i64),
            opt(newline)
        )(i)
    }

    fn term(i: &str) -> IResult<&str, Term> {
        alt((
            map(tag("old"), |_| Term::Old),
            map(nom::character::complete::i64, Term::Const)
        ))(i)
    }

    fn expr(i: &str) -> IResult<&str, Op> {
        alt((
            map(separated_pair(term, tag(" * "), term), |(t1, t2)| Op::Mult(t1, t2)),
            map(separated_pair(term, tag(" + "), term), |(t1, t2)| Op::Add(t1, t2)),
        ))(i)
    }

    fn parse_op(i: &str) -> IResult<&str, Op> {
        delimited(
            tuple((space1, tag("Operation: new = "))),
            expr,
            opt(newline)
        )(i)
    }

    fn parse_test(i: &str) -> IResult<&str, WorryTest> {
        let pdiv = delimited(
            tuple((space1, tag("Test: divisible by "))),
            nom::character::complete::i64,
            opt(newline)
        );

        let ptrue = delimited(
            tuple((space1, tag("If true: throw to monkey "))),
            nom::character::complete::i64,
            opt(newline)
        );

        let pfalse = delimited(
            tuple((space1, tag("If false: throw to monkey "))),
            nom::character::complete::i64,
            opt(newline)
        );

        map(
            tuple((pdiv, ptrue, pfalse)),
            |(d, t, f)| WorryTest { divisor: d, when_true: t, when_false: f}
        )(i)
    }

    pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
        let monkey_line = tuple((tag("Monkey "), nom::character::complete::u64, tag(":"), newline));
        map(
            tuple((monkey_line, parse_items, parse_op, parse_test)),
            |(_, i, o, t)| Monkey { items: VecDeque::from(i), op: o, test: t, inspections: 0}
        )(i)
    }
}