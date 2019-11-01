use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

fn solve(input: &str) -> u16 {
    let mut circuit = Circuit::new();

    input
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .for_each(|c| circuit.add(c));

    circuit.get("a").unwrap()
}

#[derive(Default)]
struct Circuit<'s> {
    values: HashMap<&'s str, u16>,
    deps: HashMap<&'s str, Vec<Instruction<'s>>>,
}

impl<'s> Circuit<'s> {
    pub fn new() -> Self {
        Circuit::default()
    }

    pub fn add(&mut self, instruction: Instruction<'s>) {
        let mut unblocked_instructions = self.raw_add(instruction);
        while !unblocked_instructions.is_empty() {
            unblocked_instructions = unblocked_instructions
                .into_iter()
                .flat_map(|i| self.raw_add(i))
                .collect();
        }
    }

    pub fn get(&self, id: &str) -> Option<u16> {
        self.values.get(id).cloned()
    }

    /// returns a list of instructions that can now be re-attempted
    fn raw_add(&mut self, instruction: Instruction<'s>) -> Vec<Instruction<'s>> {
        let input = match instruction.input {
            Signal::Literal(l) => l,
            Signal::Variable(v) => match self.values.get(v) {
                None => {
                    self.deps.entry(v).or_default().push(instruction);
                    return vec![];
                }
                Some(&l) => l,
            },
        };

        match instruction.gate {
            Gate::Unary { operation } => {
                let input = match operation {
                    UnaryOperation::Negate => !input,
                    UnaryOperation::Wire => input,
                };
                self.values.insert(instruction.output, input);
            }
            Gate::Binary { operation, input_b } => {
                let input_b = match input_b {
                    Signal::Literal(l) => l,
                    Signal::Variable(v) => match self.values.get(v) {
                        None => {
                            self.deps.entry(v).or_default().push(instruction);
                            return vec![];
                        }
                        Some(&l) => l,
                    },
                };
                match operation {
                    BinaryOperation::And => {
                        self.values.insert(instruction.output, input & input_b);
                    }
                    BinaryOperation::Or => {
                        self.values.insert(instruction.output, input | input_b);
                    }
                }
            }
            Gate::Shift { operation, shift } => match operation {
                ShiftOperation::Left => {
                    self.values.insert(instruction.output, input << shift);
                }
                ShiftOperation::Right => {
                    self.values.insert(instruction.output, input >> shift);
                }
            },
        };

        self.deps.remove(instruction.output).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'s> {
    input: Signal<'s>,
    output: &'s str,
    gate: Gate<'s>,
}

#[derive(Debug, Clone, Copy)]
enum Gate<'s> {
    Shift {
        operation: ShiftOperation,
        shift: u8,
    },
    Unary {
        operation: UnaryOperation,
    },
    Binary {
        input_b: Signal<'s>,
        operation: BinaryOperation,
    },
}

#[derive(Debug, Clone, Copy)]
enum BinaryOperation {
    Or,
    And,
}

#[derive(Debug, Clone, Copy)]
enum ShiftOperation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum UnaryOperation {
    Wire,
    Negate,
}

#[derive(Clone, Copy, Debug)]
enum Signal<'s> {
    Literal(u16),
    Variable(&'s str),
}

impl<'s> From<&'s str> for Signal<'s> {
    fn from(input: &'s str) -> Self {
        input
            .parse::<u16>()
            .map(Signal::Literal)
            .unwrap_or_else(|_| Signal::Variable(input))
    }
}

impl<'s> TryFrom<&'s str> for Instruction<'s> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &'s str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
            Regex::new(r"^(?P<not>NOT )?(?P<in>[^\s]+)(?: (?P<operation>OR|AND|RSHIFT|LSHIFT) (?P<in_b>[^\s]+))? -> (?P<out>[a-z]+)$").unwrap();
        }

        let captures = RE.captures(input).ok_or_else(|| "did not match regex")?;
        let output = captures.name("out").unwrap().as_str();
        let input: Signal = captures.name("in").unwrap().as_str().try_into()?;

        let gate = captures
            .name("operation")
            .map(|op| -> Result<_, Box<dyn std::error::Error>> {
                let input_b = captures.name("in_b").unwrap().as_str();
                Ok(match op.as_str() {
                    "OR" => Gate::Binary {
                        input_b: input_b.into(),
                        operation: BinaryOperation::Or,
                    },
                    "AND" => Gate::Binary {
                        input_b: input_b.into(),
                        operation: BinaryOperation::And,
                    },
                    "LSHIFT" => Gate::Shift {
                        shift: input_b.parse()?,
                        operation: ShiftOperation::Left,
                    },
                    "RSHIFT" => Gate::Shift {
                        shift: input_b.parse()?,
                        operation: ShiftOperation::Right,
                    },
                    _ => unreachable!(),
                })
            })
            .unwrap_or_else(|| {
                Ok(match captures.name("not") {
                    None => Gate::Unary {
                        operation: UnaryOperation::Wire,
                    },
                    Some(_) => Gate::Unary {
                        operation: UnaryOperation::Negate,
                    },
                })
            })?;

        Ok(Instruction {
            output,
            input,
            gate,
        })
    }
}

#[cfg(test)]
mod seven_a {
    use super::*;

    #[test]
    fn test() {
        let input = r"456 -> y
x AND y -> d
x OR y -> e
NOT x -> a
x LSHIFT 2 -> f
y RSHIFT 2 -> g
123 -> x
NOT y -> i";
        assert_eq!(solve(input), 65412);
    }
}

common::read_main!();
