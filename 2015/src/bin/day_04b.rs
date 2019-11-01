fn solve(input: &str) -> usize {
    (0..)
        .map(|i| format!("{}{}", input.trim(), i))
        .map(|data| md5::compute(data))
        .take_while(|hash| hash[0] != 0 || hash[1] != 0 || hash[2] != 0)
        .count()
}

common::read_main!();
