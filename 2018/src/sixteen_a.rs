use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;
use strum::IntoEnumIterator;

fn solve(input: &str) -> usize {
    input
        .split_terminator("\n\n")
        .take_while(|s| !s.is_empty())
        .map(|l| Sample::from_str(l).unwrap())
        .filter(|s| {
            let [_, a, b, c] = s.instruction;
            Instruction::iter()
                .map(|i| s.before.calculate(i, a, b, c))
                .filter(|r| *r == s.after)
                .take(3)
                .count()
                == 3
        })
        .count()
}

#[derive(Debug)]
struct Sample {
    before: Registers,
    instruction: [u8; 4],
    after: Registers,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Registers([u8; 4]);

impl Registers {
    fn calculate(mut self, instruction: Instruction, a: u8, b: u8, c: u8) -> Registers {
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
            Instruction::GtIR => (a > self.0[b as usize]) as u8,
            Instruction::GtRI => (self.0[a as usize] > b) as u8,
            Instruction::GtRR => (self.0[a as usize] > self.0[b as usize]) as u8,
            Instruction::EqIR => (a == self.0[b as usize]) as u8,
            Instruction::EqRI => (self.0[a as usize] == b) as u8,
            Instruction::EQRR => (self.0[a as usize] == self.0[b as usize]) as u8,
        };
        self
    }
}

#[derive(strum_macros::EnumIter, Clone, Copy)]
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
        let b0: u8 = caps.name("b0").ok_or(ParsingError)?.as_str().parse()?;
        let b1: u8 = caps.name("b1").ok_or(ParsingError)?.as_str().parse()?;
        let b2: u8 = caps.name("b2").ok_or(ParsingError)?.as_str().parse()?;
        let b3: u8 = caps.name("b3").ok_or(ParsingError)?.as_str().parse()?;
        let i0: u8 = caps.name("i0").ok_or(ParsingError)?.as_str().parse()?;
        let i1: u8 = caps.name("i1").ok_or(ParsingError)?.as_str().parse()?;
        let i2: u8 = caps.name("i2").ok_or(ParsingError)?.as_str().parse()?;
        let i3: u8 = caps.name("i3").ok_or(ParsingError)?.as_str().parse()?;
        let a0: u8 = caps.name("a0").ok_or(ParsingError)?.as_str().parse()?;
        let a1: u8 = caps.name("a1").ok_or(ParsingError)?.as_str().parse()?;
        let a2: u8 = caps.name("a2").ok_or(ParsingError)?.as_str().parse()?;
        let a3: u8 = caps.name("a3").ok_or(ParsingError)?.as_str().parse()?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        assert_eq!(solve(input), 1);
    }
}

common::read_main!();
