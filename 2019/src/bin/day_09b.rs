use intcode::{Intcode, Machine, MachineResult};

fn solve(input: &str) -> i64 {
    let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    let mut machine = Machine::new(codes);

    match machine.execute() {
        MachineResult::AwaitingInput(ai) => machine = ai.provide(2),
        _ => unreachable!(),
    }

    let mut last_output = 0;
    while let MachineResult::HasOutput(output) = machine.execute() {
        let (output, m) = output.read();
        println!("output: {}", output);
        last_output = output;
        machine = m;
    }

    last_output
}

common::read_main!();
