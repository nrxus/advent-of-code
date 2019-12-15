use intcode::{Machine, MachineResult};
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut machine = Machine::from_str(input);
    let mut blocks = HashSet::new();

    loop {
        let mut pos = (0, 0);
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (x, m) = ho.read();
                pos.0 = x;
                machine = m;
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected input request"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (y, m) = ho.read();
                pos.1 = y;
                machine = m;
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected input request"),
        }
        match machine.execute() {
            MachineResult::HasOutput(ho) => {
                let (tile, m) = ho.read();
                if tile == 2 {
                    blocks.insert(pos);
                }
                machine = m;
            }
            MachineResult::Halted(_) => break,
            _ => panic!("unexpected input request"),
        }
    }

    blocks.len()
}

common::read_main!();
