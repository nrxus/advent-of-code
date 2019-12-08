use intcode::{Intcode, Machine, MachineResult};

fn solve(input: &str) -> i32 {
    let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    let mut machine = Machine::new(codes);
    let mut input = Some(5);
    let mut last_output = None;

    loop {
        match machine.execute() {
            MachineResult::AwaitingInput(ai) => machine = ai.provide(input.take().unwrap()),
            MachineResult::HasOutput(output) => {
                let (output, m) = output.read();
                if output != 0 {
                    last_output = Some(output);
                    println!("diagnostic result: {}", output);
                }
                machine = m;
            }
            MachineResult::Halted(_) => break,
        }
    }

    last_output.unwrap()
}

common::read_main!();
