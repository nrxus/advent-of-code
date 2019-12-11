use intcode::{Intcode, Machine};

fn solve(input: &str) -> i64 {
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

fn first_after_run(codes: Vec<Intcode>) -> i64 {
    Machine::new(codes)
        .run_to_halt()
        .expect("did not halt")
        .get(0)
        .0
}

common::read_main!();
