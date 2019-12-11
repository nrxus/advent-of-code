use intcode::{Intcode, Machine, MachineResult};
use itertools::Itertools;

fn solve(input: &str) -> i64 {
    let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    let permutations = (0..5).permutations(5);

    permutations
        .map(|mut phase_sequence| {
            let mut input = 0;
            for _ in 0..5 {
                let mut inputs = vec![input, phase_sequence.pop().unwrap()];
                let mut machine = Machine::new(codes.clone());
                let output = loop {
                    match machine.execute() {
                        MachineResult::AwaitingInput(ai) => {
                            machine = ai.provide(inputs.pop().unwrap())
                        }
                        MachineResult::HasOutput(output) => break output.read().0,
                        MachineResult::Halted(_) => panic!("program ended without output"),
                    }
                };

                input = output;
            }
            input
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(solve(input), 43210);

        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(solve(input), 54321);

        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(solve(input), 65210);
    }
}

common::read_main!();
