use intcode::{Machine, MachineResult};
use std::{collections::HashMap, fmt::Write};

fn solve(input: &str) -> Paint {
    let mut machine = Machine::from_str(input);
    let mut painted = HashMap::new();
    let mut position = (0, 0);
    painted.insert(position, Color::White);
    let mut direction = Direction::Up;

    loop {
        match machine.execute() {
            MachineResult::AwaitingInput(ai) => {
                let input = painted.get(&position).cloned().unwrap_or(Color::Black);
                machine = ai.provide(input as i64);
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected output from machine"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (color, m) = ho.read();
                painted.insert(
                    position,
                    match color {
                        0 => Color::Black,
                        1 => Color::White,
                        _ => panic!("only 0 or 1 expected for colors"),
                    },
                );
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

    Paint(painted)
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

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => f.write_char('■'),
            Color::White => f.write_char('□'),
        }
    }
}

struct Paint(HashMap<(isize, isize), Color>);

impl std::fmt::Display for Paint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.keys().map(|&(x, _)| x).min().unwrap();
        let max_x = self.0.keys().map(|&(x, _)| x).max().unwrap();
        let min_y = self.0.keys().map(|&(_, y)| y).min().unwrap();
        let max_y = self.0.keys().map(|&(_, y)| y).max().unwrap();

        (min_y..=max_y).try_for_each(|y| {
            (min_x..=max_x)
                .try_for_each(|x| write!(f, "{}", self.0.get(&(x, y)).unwrap_or(&Color::Black)))?;
            f.write_char('\n')
        })
    }
}

common::read_main!();
