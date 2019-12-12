use intcode::{Machine, MachineResult};
use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let mut machine = Machine::from_str(input);
    let mut painted = HashMap::new();
    let mut position = (0, 0);
    let mut direction = Direction::Up;

    loop {
        match machine.execute() {
            MachineResult::AwaitingInput(ai) => {
                let input = painted.get(&position).cloned().unwrap_or(0);
                machine = ai.provide(input);
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected output from machine"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (color, m) = ho.read();
                painted.insert(position, color);
                machine = m;
            }
            _ => panic!("expected the machine to need input"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (turn, m) = ho.read();
                let turn = match turn {
                    0 => Turn::Left,
                    1 => Turn::Right,
                    _ => panic!("unexpected direction"),
                };
                direction.turn(turn);
                match direction {
                    Direction::Up => position.1 -= 1,
                    Direction::Down => position.1 += 1,
                    Direction::Left => position.0 -= 1,
                    Direction::Right => position.0 += 1,
                }
                machine = m;
            }
            _ => panic!("expected the machine to need input"),
        }
    }

    painted.len()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Turn {
    Left,
    Right,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        *self = match (&self, turn) {
            (Direction::Up, Turn::Left) | (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Up, Turn::Right) | (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Left, Turn::Left) | (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Left, Turn::Right) | (Direction::Right, Turn::Left) => Direction::Up,
        }
    }
}

common::read_main!();
