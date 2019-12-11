use intcode::{Intcode, Machine, MachineResult};
use itertools::Itertools;

fn solve(input: &str) -> i64 {
    let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    let permutations = (5..10).permutations(5);

    permutations
        .map(|phase_sequence| {
            let mut input = 0;
            let mut a = match Machine::new(codes.clone()).execute() {
                MachineResult::AwaitingInput(ai) => ai.provide(phase_sequence[0]),
                _ => panic!("expected the need for input"),
            };
            let mut b = match Machine::new(codes.clone()).execute() {
                MachineResult::AwaitingInput(ai) => ai.provide(phase_sequence[1]),
                _ => panic!("expected the need for input"),
            };
            let mut c = match Machine::new(codes.clone()).execute() {
                MachineResult::AwaitingInput(ai) => ai.provide(phase_sequence[2]),
                _ => panic!("expected the need for input"),
            };
            let mut d = match Machine::new(codes.clone()).execute() {
                MachineResult::AwaitingInput(ai) => ai.provide(phase_sequence[3]),
                _ => panic!("expected the need for input"),
            };
            let mut e = match Machine::new(codes.clone()).execute() {
                MachineResult::AwaitingInput(ai) => ai.provide(phase_sequence[4]),
                _ => panic!("expected the need for input"),
            };

            loop {
                a = match a.execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(input),
                    MachineResult::Halted(_) => return input,
                    _ => panic!("unexpected output before input"),
                };
                match a.execute() {
                    MachineResult::HasOutput(output) => {
                        let (i, machine) = output.read();
                        input = i;
                        a = machine
                    }
                    _ => panic!("expected output"),
                };

                b = match b.execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(input),
                    _ => panic!("expected the need for input"),
                };
                match b.execute() {
                    MachineResult::HasOutput(output) => {
                        let (i, machine) = output.read();
                        input = i;
                        b = machine
                    }
                    _ => panic!("expected output"),
                };

                c = match c.execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(input),
                    _ => panic!("expected the need for input"),
                };
                match c.execute() {
                    MachineResult::HasOutput(output) => {
                        let (i, machine) = output.read();
                        input = i;
                        c = machine
                    }
                    _ => panic!("expected output"),
                };

                d = match d.execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(input),
                    _ => panic!("expected the need for input"),
                };
                match d.execute() {
                    MachineResult::HasOutput(output) => {
                        let (i, machine) = output.read();
                        input = i;
                        d = machine
                    }
                    _ => panic!("expected output"),
                };

                e = match e.execute() {
                    MachineResult::AwaitingInput(ai) => ai.provide(input),
                    _ => panic!("expected the need for input"),
                };
                match e.execute() {
                    MachineResult::HasOutput(output) => {
                        let (i, machine) = output.read();
                        input = i;
                        e = machine
                    }
                    _ => panic!("expected output"),
                };
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(solve(input), 139629729);

        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(solve(input), 18216);
    }
}

common::read_main!();
