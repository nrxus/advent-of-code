use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

use lazy_static::lazy_static;
use regex::Regex;
use strum::IntoEnumIterator;

fn solve(input: &str) -> u16 {
    let mut input = input.split_terminator("\n\n\n");

    //calculate all possible opcodes -> Instructions independently
    let samples = input.next().expect("no samples found");
    let mut possibilities: HashMap<u16, HashSet<Instruction>> = samples
        .trim()
        .split_terminator("\n\n")
        .map(|l| Sample::from_str(l).unwrap())
        .map(|s| {
            let [n, a, b, c] = s.instruction;
            let possibilities = Instruction::iter()
                .filter(|&i| s.before.calculate(i, a, b, c) == s.after)
                .collect();
            (n, possibilities)
        })
        .fold(HashMap::new(), |mut r, (n, possibilities)| {
            r.entry(n)
                .and_modify(|p| {
                    *p = p.intersection(&possibilities).cloned().collect();
                })
                .or_insert(possibilities);
            r
        });

    //collapse each opcode to a single instruction
    let mut result = HashMap::<u16, Instruction>::new();
    while result.len() < 16 {
        let found: Vec<_> = possibilities
            .iter()
            .filter(|(_, p)| p.len() == 1)
            .map(|(&code, instruction)| (code, instruction.iter().nth(0).cloned().unwrap()))
            .collect();
        found.into_iter().for_each(|(code, instruction)| {
            possibilities.values_mut().for_each(|p| {
                p.remove(&instruction);
            });
            result.insert(code, instruction);
        });
    }

    let program = input.next().expect("no test program found");
    let registers = program
        .trim()
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let i = words.next().unwrap().parse().map(|i| result[&i]).unwrap();
            let a: u16 = words.next().unwrap().parse().unwrap();
            let b: u16 = words.next().unwrap().parse().unwrap();
            let c: u16 = words.next().unwrap().parse().unwrap();
            (i, a, b, c)
        })
        .fold(Registers([0, 0, 0, 0]), |r, (i, a, b, c)| {
            r.calculate(i, a, b, c)
        });
    registers.0[0]
}

#[derive(Debug)]
struct Sample {
    before: Registers,
    instruction: [u16; 4],
    after: Registers,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Registers([u16; 4]);

impl Registers {
    fn calculate(mut self, instruction: Instruction, a: u16, b: u16, c: u16) -> Registers {
        self.0[c as usize] = match instruction {
            Instruction::AddR => self.0[a as usize] + self.0[b as usize],
            Instruction::AddI => self.0[a as usize] + b,
            Instruction::MulR => self.0[a as usize] * self.0[b as usize],
            Instruction::MulI => self.0[a as usize] * b,
            Instruction::BandR => self.0[a as usize] & self.0[b as usize],
            Instruction::BandI => self.0[a as usize] & b,
            Instruction::BorR => self.0[a as usize] | self.0[b as usize],
            Instruction::BorI => self.0[a as usize] | b,
            Instruction::SetR => self.0[a as usize],
            Instruction::SetI => a,
            Instruction::GtIR => (a > self.0[b as usize]) as u16,
            Instruction::GtRI => (self.0[a as usize] > b) as u16,
            Instruction::GtRR => (self.0[a as usize] > self.0[b as usize]) as u16,
            Instruction::EqIR => (a == self.0[b as usize]) as u16,
            Instruction::EqRI => (self.0[a as usize] == b) as u16,
            Instruction::EQRR => (self.0[a as usize] == self.0[b as usize]) as u16,
        };
        self
    }
}

#[derive(strum_macros::EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Instruction {
    AddR,
    AddI,
    MulR,
    MulI,
    BandR,
    BandI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EQRR,
}

use std::str::FromStr;

impl FromStr for Sample {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Before:\s+\[(?P<b0>\d+), (?P<b1>\d+), (?P<b2>\d+), (?P<b3>\d+)\]
(?P<i0>\d+) (?P<i1>\d+) (?P<i2>\d+) (?P<i3>\d+)
After:\s+\[(?P<a0>\d+), (?P<a1>\d+), (?P<a2>\d+), (?P<a3>\d+)\]"
            )
            .unwrap();
        }

        let caps = RE.captures(input).ok_or(ParsingError)?;
        let b0: u16 = caps.name("b0").ok_or(ParsingError)?.as_str().parse()?;
        let b1: u16 = caps.name("b1").ok_or(ParsingError)?.as_str().parse()?;
        let b2: u16 = caps.name("b2").ok_or(ParsingError)?.as_str().parse()?;
        let b3: u16 = caps.name("b3").ok_or(ParsingError)?.as_str().parse()?;
        let i0: u16 = caps.name("i0").ok_or(ParsingError)?.as_str().parse()?;
        let i1: u16 = caps.name("i1").ok_or(ParsingError)?.as_str().parse()?;
        let i2: u16 = caps.name("i2").ok_or(ParsingError)?.as_str().parse()?;
        let i3: u16 = caps.name("i3").ok_or(ParsingError)?.as_str().parse()?;
        let a0: u16 = caps.name("a0").ok_or(ParsingError)?.as_str().parse()?;
        let a1: u16 = caps.name("a1").ok_or(ParsingError)?.as_str().parse()?;
        let a2: u16 = caps.name("a2").ok_or(ParsingError)?.as_str().parse()?;
        let a3: u16 = caps.name("a3").ok_or(ParsingError)?.as_str().parse()?;

        Ok(Sample {
            before: Registers([b0, b1, b2, b3]),
            instruction: [i0, i1, i2, i3],
            after: Registers([a0, a1, a2, a3]),
        })
    }
}

#[derive(Debug)]
struct ParsingError;

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

common::read_main!();
