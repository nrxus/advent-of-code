use intcode::{Intcode, Machine, MachineResult};

fn solve(input: &str) -> i32 {
    let mut codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();

    for verb in 0..=99 {
        for noun in 0..=99 {
            codes[1] = Intcode(noun);
            codes[2] = Intcode(verb);

            let result = first_after_run(codes.clone());
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!();
}

fn first_after_run(codes: Vec<Intcode>) -> i32 {
    match Machine::new(codes).execute() {
        MachineResult::Halted(codes) => codes[0].0,
        _ => panic!("program did not halt correctly"),
    }
}

common::read_main!();
