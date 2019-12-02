fn solve(input: &str) -> usize {
    let mut codes = input
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    codes[1] = 12;
    codes[2] = 2;

    intcode(codes)
}

fn intcode(mut codes: Vec<usize>) -> usize {
    let mut current = 0;
    while codes[current] != 99 {
        let result = match codes[current] {
            1 => codes[codes[current + 1]] + codes[codes[current + 2]],
            2 => codes[codes[current + 1]] * codes[codes[current + 2]],
            _ => unreachable!(),
        };
        let location = codes[current + 3];
        codes[location] = result;
        current += 4;
    }
    codes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(intcode(vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(intcode(vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(intcode(vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        assert_eq!(
            intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            3500
        );
    }
}

common::read_main!();
