use enum_map::{enum_map, Enum};
use intcode::{Machine, MachineResult};
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut frontier = vec![Step {
        machine: Machine::from_str(input),
        position: (0, 0),
        steps: 0,
    }];
    let mut explored = HashSet::new();

    while let Some(step) = frontier.pop() {
        let new_frontier = enum_map! {
            direction => match step.machine.clone().execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(direction as i64),
                    _ => panic!("expected the need of input"),
                }
        };

        let new_frontier =
            new_frontier
                .into_iter()
                .filter_map(|(direction, machine)| match machine.execute() {
                    MachineResult::HasOutput(ho) => {
                        let (output, machine) = ho.read();
                        if output == 0 {
                            None
                        } else {
                            let position = match direction {
                                Direction::North => (step.position.0, step.position.1 + 1),
                                Direction::South => (step.position.0, step.position.1 - 1),
                                Direction::East => (step.position.0 - 1, step.position.1),
                                Direction::West => (step.position.0 + 1, step.position.1),
                            };
                            if explored.contains(&position) {
                                None
                            } else {
                                explored.insert(position);
                                Some((
                                    Step {
                                        position,
                                        machine,
                                        steps: step.steps + 1,
                                    },
                                    output == 2,
                                ))
                            }
                        }
                    }
                    _ => panic!("expected to get output"),
                });

        for (step, done) in new_frontier {
            if done {
                return step.steps;
            }
            frontier.push(step);
        }
    }

    panic!("Did not find a path to the oxygen system");
}

struct Step {
    machine: Machine,
    position: (i32, i32),
    steps: usize,
}

#[derive(Enum)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

common::read_main!();
