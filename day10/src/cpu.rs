use std::collections::HashSet;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, terminated},
    IResult
};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add(i64),
    Noop,
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CpuState {
    cycles: usize,
    register_value: i64
}

impl CpuState {
    pub fn run_to(instructions: &Vec<Instruction>, cycle_limit: usize) -> Self {
        let mut register_value = 1;
        let mut cycles = 0;

        for i in instructions {
            cycles += i.cycles();

            if cycles < cycle_limit {
                if let Instruction::Add(n) = i {
                    register_value += n;
                }
            }
        }

        Self { register_value, cycles: cycle_limit }
    }

    pub fn signal_strength(&self) -> i64 {
        (self.cycles as i64) * self.register_value
    }

    pub fn update(&mut self, inst: Instruction) {
        self.cycles += inst.cycles();
        if let Instruction::Add(n) = inst {
            self.register_value += n;
        }
    }

    pub fn cycle_states(inst_set: &Vec<Instruction>) -> Vec<CpuState> {
        let total_cycles = inst_set.iter().map(|i| i.cycles()).sum();
        let mut states = Vec::<CpuState>::with_capacity(total_cycles);

        states.push(CpuState::default());
        for &ins in inst_set {
            let mut prev = states.last().unwrap().clone();
            match ins {
                Instruction::Noop => {
                    prev.cycles += 1;
                    states.push(prev);
                },
                Instruction::Add(n) => {
                    prev.cycles += 1;
                    states.push(prev.clone());

                    prev.cycles += 1;
                    prev.register_value += n;
                    states.push(prev.clone());
                }
            }
        }

        states
    }
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            register_value: 1,
            cycles: 0,
        }
    }
}

impl fmt::Debug for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cycles={}, register={}", self.cycles, self.register_value)
    }
}

pub struct Screen {
    cpu_states: Vec<CpuState>
}

impl Screen {
    pub fn new(instructions: &Vec<Instruction>) -> Self {
        Screen {
            cpu_states: CpuState::cycle_states(instructions)
        }
    }

    pub fn render(&self) {
        for row in (0..6) {
            for col in (0..40) {
                let idx = col + 40 * row;
                let delta = self.cpu_states[idx].register_value - (col as i64);
                if delta.abs() <= 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub mod parser {
    use super::*;

    pub type PResult<'a, T> = IResult<&'a str, T>;

    fn parse_instruction(i: &str) -> PResult<Instruction> {
        alt((
            map(tag("noop"), |_| Instruction::Noop),
            map(preceded(tag("addx "), nom::character::complete::i64), |n| Instruction::Add(n))
        ))(i)
    }

    pub fn parse_input(i: &str) -> PResult<Vec<Instruction>> {
        many1(terminated(parse_instruction, opt(nom::character::complete::newline)))(i)
    }
}