fn solve(input: &str) -> u32 {
    let signal = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let output_signal = (0..100).fold(signal, |signal, _| step(signal));

    output_signal
        .into_iter()
        .take(8)
        .scan(10_000_000, |multiplier, digit| {
            let num = *multiplier * digit as u32;
            *multiplier /= 10;
            Some(num)
        })
        .sum()
}

fn step(signal: Vec<u8>) -> Vec<u8> {
    let pattern = [0, 1, 0, -1];

    (0..signal.len())
        .map(|i| {
            let pattern = pattern
                .iter()
                .flat_map(|p| std::iter::repeat(p).take(i + 1))
                .cycle()
                .skip(1);

            let output: i32 = signal
                .iter()
                .zip(pattern)
                .map(|(&digit, pattern)| (digit as i32 * pattern))
                .sum();

            (output.abs() % 10) as u8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let output = step(input);
        assert_eq!(output, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        let output = step(output);
        assert_eq!(output, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        let output = step(output);
        assert_eq!(output, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        let output = step(output);
        assert_eq!(output, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn larger() {
        let input = r"80871224585914546619083218645595";
        assert_eq!(solve(input), 24176176);

        let input = r"19617804207202209144916044189917";
        assert_eq!(solve(input), 73745418);

        let input = r"69317163492948606335995924319873";
        assert_eq!(solve(input), 52432133);
    }
}

common::read_main!();
