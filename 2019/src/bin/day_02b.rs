fn solve(input: &str) -> usize {
    let mut codes = input
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for verb in 0..=99 {
        for noun in 0..=99 {
            codes[1] = noun;
            codes[2] = verb;

            let result = intcode(codes.clone());
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!();
}

fn intcode(mut codes: Vec<usize>) -> usize {
    let mut ip = 0;
    while codes[ip] != 99 {
        let result = match codes[ip] {
            1 => codes[codes[ip + 1]] + codes[codes[ip + 2]],
            2 => codes[codes[ip + 1]] * codes[codes[ip + 2]],
            _ => unreachable!(),
        };
        let output_address = codes[ip + 3];
        codes[output_address] = result;
        ip += 4;
    }
    codes[0]
}

common::read_main!();
