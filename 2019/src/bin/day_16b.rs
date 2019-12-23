fn solve(input: &str) -> u32 {
    let input = input.trim();
    let offset: usize = input[0..7].parse().unwrap();

    if offset < input.len() * 10_000 / 2 {
        panic!("could not run optimized FFT; offset is too small")
    }

    let mut signal: Vec<u8> = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .cycle()
        .take(input.len() * 10_000)
        .skip(offset)
        .collect();

    for _ in 0..100 {
        for i in (0..signal.len()).rev() {
            signal[i] = (signal[i] + signal.get(i + 1).cloned().unwrap_or(0)) % 10;
        }
    }

    signal
        .into_iter()
        .take(8)
        .scan(10_000_000, |multiplier, digit| {
            let num = *multiplier * digit as u32;
            *multiplier /= 10;
            Some(num)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_a() {
        let input = r"03036732577212944063491565474664";
        assert_eq!(solve(input), 84462026);
    }

    #[test]
    fn larger_b() {
        let input = r"02935109699940807407585447034323";
        assert_eq!(solve(input), 78725270);
    }

    #[test]
    fn larger_c() {
        let input = r"03081770884921959731165446850517";
        assert_eq!(solve(input), 53553731);
    }
}

common::read_main!();
