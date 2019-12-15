use intcode::{Intcode, Machine, MachineResult};
use std::{collections::HashMap, fmt::Write};

fn solve(input: &str) -> u32 {
    let mut codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    codes[0] = Intcode(2);
    let mut machine = Machine::new(codes);
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut tiles = HashMap::new();
    let mut score = 0;

    loop {
        let x;
        let y;
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (out, m) = ho.read();
                x = out;
                machine = m;
            }
            MachineResult::AwaitingInput(ai) => {
                let game = Game {
                    score,
                    tiles: tiles.clone(),
                };
                let cmp = ball_x.cmp(&paddle_x);
                machine = ai.provide(cmp as i64);
                println!("{}", game);
                match machine.execute() {
                    MachineResult::HasOutput(ho) => {
                        let (out, m) = ho.read();
                        x = out;
                        machine = m;
                    }
                    _ => unreachable!(),
                }
            }
            MachineResult::Halted(_) => break,
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (out, m) = ho.read();
                y = out;
                machine = m;
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected input request"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (output, m) = ho.read();
                if x == -1 && y == 0 {
                    score = output as u32;
                } else {
                    let pos = (x as u32, y as u32);
                    match output {
                        0 => tiles.remove(&pos),
                        1 => tiles.insert(pos, Tile::Wall),
                        2 => tiles.insert(pos, Tile::Block),
                        3 => {
                            paddle_x = pos.0;
                            tiles.insert(pos, Tile::Paddle)
                        }
                        4 => {
                            ball_x = pos.0;
                            tiles.insert(pos, Tile::Ball)
                        }
                        _ => unreachable!(),
                    };
                }

                machine = m;
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected input request"),
        }
    }

    score
}

struct Game {
    score: u32,
    tiles: HashMap<(u32, u32), Tile>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.tiles.keys().map(|&(x, _)| x).max().unwrap();
        let max_y = self.tiles.keys().map(|&(_, y)| y).max().unwrap();

        (0..=max_y).try_for_each(|y| {
            (0..=max_x).try_for_each(|x| match self.tiles.get(&(x, y)) {
                None => f.write_char(' '),
                Some(t) => write!(f, "{}", t),
            })?;
            f.write_char('\n')
        })?;

        write!(f, "score: {}", self.score)
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Block,
    Paddle,
    Ball,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => '█',
            Tile::Block => '◫',
            Tile::Paddle => '▬',
            Tile::Ball => '◯',
        };
        f.write_char(c)
    }
}

common::read_main!();
