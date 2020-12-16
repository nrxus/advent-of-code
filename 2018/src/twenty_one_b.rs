use std::{num::ParseIntError, str::FromStr};

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
        let mut cache = vec![];

        while self.instruction_index() < self.lines.len() {
            let line = &self.lines[self.instruction_index()];
            if self.registers.calculate(line) {
                if cache.contains(&self.registers.0) {
                    break;
                } else {
                    cache.push(self.registers.0);
                }
            }
            self.registers.0[self.ip] += 1;
        }

        let mut values: Vec<_> = cache.into_iter().map(|reg| reg[3]).collect();

        while let Some(v) = values.pop() {
            if !values.contains(&v) {
                return v;
            }
        }

        panic!("WHAAAT");
    }

    fn instruction_index(&self) -> usize {
        self.registers.0[self.ip] as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    instruction: Instruction,
    dst: usize,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Registers([u32; 6]);

impl Registers {
    fn calculate(&mut self, line: &Line) -> bool {
        self.0[line.dst] = match line.instruction {
            Instruction::AddR(a, b) if a != 0 && b != 0 => self.0[a] + self.0[b],
            Instruction::MulR(a, b) if a != 0 && b != 0 => self.0[a] * self.0[b],
            Instruction::BanR(a, b) if a != 0 && b != 0 => self.0[a] & self.0[b],
            Instruction::BorR(a, b) if a != 0 && b != 0 => self.0[a] | self.0[b],
            Instruction::GtRR(a, b) if a != 0 && b != 0 => (self.0[a] > self.0[b]) as u32,
            Instruction::EqRR(a, b) if a != 0 && b != 0 => (self.0[a] == self.0[b]) as u32,
            Instruction::AddI(a, b) if a != 0 => self.0[a] + b,
            Instruction::MulI(a, b) if a != 0 => self.0[a] * b,
            Instruction::BanI(a, b) if a != 0 => self.0[a] & b,
            Instruction::BorI(a, b) if a != 0 => self.0[a] | b,
            Instruction::GtRI(a, b) if a != 0 => (self.0[a] > b) as u32,
            Instruction::EqRI(a, b) if a != 0 => (self.0[a] == b) as u32,
            Instruction::GtIR(a, b) if b != 0 => (a > self.0[b]) as u32,
            Instruction::EqIR(a, b) if b != 0 => (a == self.0[b]) as u32,
            Instruction::SetR(a) if a != 0 => self.0[a],
            Instruction::SetI(a) => a,
            _ => {
                self.0[1] = 0;
                return true;
            }
        };

        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            .captures(input.next().ok_or(ParsingError)?)
            .ok_or(ParsingError)?
            .name("ip")
            .ok_or(ParsingError)?
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

        let caps = RE.captures(input).ok_or(ParsingError)?;
        let instruction = caps.name("instruction").ok_or(ParsingError)?.as_str();
        let a = caps.name("a").ok_or(ParsingError)?.as_str();
        let b = caps.name("b").ok_or(ParsingError)?.as_str();
        let dst: usize = caps.name("c").ok_or(ParsingError)?.as_str().parse()?;

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

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

common::read_main!();
