#![feature(try_trait)]

use std::{num::ParseIntError, option::NoneError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    Program::from_str(input).unwrap().run()
}

#[derive(Debug)]
struct Program {
    lines: Vec<Line>,
    registers: Registers,
    ip: usize,
}

impl Program {
    fn run(&mut self) -> u32 {
        while self.instruction_index() < self.lines.len() {
            let line = &self.lines[self.instruction_index()];
            self.registers.calculate(line);
            self.registers.0[self.ip] += 1;
        }

        self.registers.0[0]
    }

    fn instruction_index(&self) -> usize {
        self.registers.0[self.ip] as usize
    }
}

#[derive(Debug)]
struct Line {
    instruction: Instruction,
    dst: usize,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Registers([u32; 6]);

impl Registers {
    fn calculate(&mut self, line: &Line) {
        self.0[line.dst] = match line.instruction {
            Instruction::AddR(a, b) => self.0[a] + self.0[b],
            Instruction::MulR(a, b) => self.0[a] * self.0[b],
            Instruction::BanR(a, b) => self.0[a] & self.0[b],
            Instruction::BorR(a, b) => self.0[a] | self.0[b],
            Instruction::GtRR(a, b) => (self.0[a] > self.0[b]) as u32,
            Instruction::EqRR(a, b) => (self.0[a] == self.0[b]) as u32,
            Instruction::AddI(a, b) => self.0[a] + b,
            Instruction::MulI(a, b) => self.0[a] * b,
            Instruction::BanI(a, b) => self.0[a] & b,
            Instruction::BorI(a, b) => self.0[a] | b,
            Instruction::GtRI(a, b) => (self.0[a] > b) as u32,
            Instruction::EqRI(a, b) => (self.0[a] == b) as u32,
            Instruction::GtIR(a, b) => (a > self.0[b]) as u32,
            Instruction::EqIR(a, b) => (a == self.0[b]) as u32,
            Instruction::SetR(a) => self.0[a],
            Instruction::SetI(a) => a,
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddR(usize, usize),
    MulR(usize, usize),
    BanR(usize, usize),
    BorR(usize, usize),
    GtRR(usize, usize),
    EqRR(usize, usize),
    AddI(usize, u32),
    MulI(usize, u32),
    BanI(usize, u32),
    BorI(usize, u32),
    GtRI(usize, u32),
    EqRI(usize, u32),
    GtIR(u32, usize),
    EqIR(u32, usize),
    SetR(usize),
    SetI(u32),
}

impl FromStr for Program {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.lines();
        let ip: usize = Regex::new(r"#ip (?P<ip>\d+)")?
            .captures(input.next()?)?
            .name("ip")?
            .as_str()
            .parse()?;
        let lines = input.map(|l| Line::from_str(l)).collect::<Result<_, _>>()?;
        let registers = Registers([0, 0, 0, 0, 0, 0]);

        Ok(Program {
            lines,
            registers,
            ip,
        })
    }
}

impl FromStr for Line {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<instruction>.*) (?P<a>\d+) (?P<b>\d+) (?P<c>\d+)").unwrap();
        }

        let caps = RE.captures(input)?;
        let instruction = caps.name("instruction")?.as_str();
        let a = caps.name("a")?.as_str();
        let b = caps.name("b")?.as_str();
        let dst: usize = caps.name("c")?.as_str().parse()?;

        let instruction = match instruction {
            "addr" => Instruction::AddR(a.parse()?, b.parse()?),
            "addi" => Instruction::AddI(a.parse()?, b.parse()?),
            "mulr" => Instruction::MulR(a.parse()?, b.parse()?),
            "muli" => Instruction::MulI(a.parse()?, b.parse()?),
            "banr" => Instruction::BanR(a.parse()?, b.parse()?),
            "bani" => Instruction::BanI(a.parse()?, b.parse()?),
            "borr" => Instruction::BorR(a.parse()?, b.parse()?),
            "bori" => Instruction::BorI(a.parse()?, b.parse()?),
            "setr" => Instruction::SetR(a.parse()?),
            "seti" => Instruction::SetI(a.parse()?),
            "gtir" => Instruction::GtIR(a.parse()?, b.parse()?),
            "gtri" => Instruction::GtRI(a.parse()?, b.parse()?),
            "gtrr" => Instruction::GtRR(a.parse()?, b.parse()?),
            "eqir" => Instruction::EqIR(a.parse()?, b.parse()?),
            "eqri" => Instruction::EqRI(a.parse()?, b.parse()?),
            "eqrr" => Instruction::EqRR(a.parse()?, b.parse()?),
            _ => Err(ParsingError)?,
        };

        Ok(Line { instruction, dst })
    }
}

#[derive(Debug)]
struct ParsingError;

impl From<regex::Error> for ParsingError {
    fn from(_: regex::Error) -> Self {
        ParsingError
    }
}

impl From<NoneError> for ParsingError {
    fn from(_: NoneError) -> Self {
        ParsingError
    }
}

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
        let input = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        assert_eq!(solve(input), 7);
    }
}

common::read_main!();
//common::bootstrap!(16);
