use std::collections::HashMap;

use common::read_main;

fn solve(input: &str) -> u16 {
    let mut circuit: HashMap<&str, Operation> = input
        .trim()
        .lines()
        .map(|i| Instruction::try_from(i).unwrap())
        .map(|i| (i.wire, i.operation))
        .collect();

    let mut wires = HashMap::<&str, u16>::new();

    loop {
        let before = circuit.len();
        circuit.retain(|&wire, operation| -> bool {
            let value = match operation {
                Operation::Assign(signal) => match signal {
                    Signal::Wire(w) => wires.get(w).copied(),
                    Signal::Value(v) => Some(*v),
                },
                Operation::Not(w) => wires.get(w).map(|v| !v),
                Operation::LShift(w, shift) => wires.get(w).map(|w| w << *shift),
                Operation::RShift(w, shift) => wires.get(w).map(|w| w >> *shift),
                Operation::And(a, b) => {
                    let values: Option<(u16, u16)> = match (a, b) {
                        (Signal::Wire(a), Signal::Wire(b)) => {
                            wires.get(a).copied().zip(wires.get(b).copied())
                        }
                        (Signal::Wire(a), Signal::Value(b)) => wires.get(a).map(|&a| (a, *b)),
                        (Signal::Value(a), Signal::Wire(b)) => wires.get(b).map(|&b| (*a, b)),
                        (Signal::Value(a), Signal::Value(b)) => Some((*a, *b)),
                    };
                    values.map(|(a, b)| a & b)
                }
                Operation::Or(a, b) => {
                    let values: Option<(u16, u16)> = match (a, b) {
                        (Signal::Wire(a), Signal::Wire(b)) => {
                            wires.get(a).copied().zip(wires.get(b).copied())
                        }
                        (Signal::Wire(a), Signal::Value(b)) => wires.get(a).map(|&a| (a, *b)),
                        (Signal::Value(a), Signal::Wire(b)) => wires.get(b).map(|&b| (*a, b)),
                        (Signal::Value(a), Signal::Value(b)) => Some((*a, *b)),
                    };
                    values.map(|(a, b)| a | b)
                }
            };

            match value {
                Some(v) => {
                    assert!(wires.insert(wire, v).is_none(), "{wire} already inserted");
                    false
                }
                None => true,
            }
        });

        let after = circuit.len();
        assert!(before != after, "{circuit:#?}");

        if let Some(v) = wires.get("a") {
            break *v;
        }
    }
}

#[derive(Debug)]
struct Instruction<'w> {
    operation: Operation<'w>,
    wire: &'w str,
}

impl<'w> TryFrom<&'w str> for Instruction<'w> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(instruction: &'w str) -> Result<Self, Self::Error> {
        let (operation, output) = instruction.split_once(" -> ").unwrap();
        let operation = operation.try_into()?;

        Ok(Instruction {
            operation,
            wire: output,
        })
    }
}

#[derive(Debug)]
enum Operation<'w> {
    Assign(Signal<'w>),
    And(Signal<'w>, Signal<'w>),
    Or(Signal<'w>, Signal<'w>),
    Not(&'w str),
    LShift(&'w str, u8),
    RShift(&'w str, u8),
}

impl<'w> TryFrom<&'w str> for Operation<'w> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(operation: &'w str) -> Result<Self, Self::Error> {
        let mut input = operation.split_whitespace();
        let first = input.next().ok_or_else(|| "empty operation".to_string())?;
        let Some(second) = input.next() else {
            return Ok(Operation::Assign(first.into()));
        };

        if first == "NOT" {
            return Ok(Operation::Not(second));
        }

        let operation = second;
        let second = input
            .next()
            .ok_or_else(|| "missing second operand".to_string())?;

        match operation {
            "AND" => Ok(Operation::And(first.into(), second.into())),
            "OR" => Ok(Operation::Or(first.into(), second.into())),
            "LSHIFT" => Ok(Operation::LShift(first, second.parse()?)),
            "RSHIFT" => Ok(Operation::RShift(first, second.parse()?)),
            op => Err(format!("invalid operation: {op}").into()),
        }
    }
}

#[derive(Debug)]
enum Signal<'w> {
    Wire(&'w str),
    Value(u16),
}

impl<'w> From<&'w str> for Signal<'w> {
    fn from(signal: &'w str) -> Self {
        signal
            .parse::<u16>()
            .map(Signal::Value)
            .unwrap_or_else(|_| Signal::Wire(signal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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

read_main!();
