use common::extensions::cart_product;
use std::collections::HashSet;

fn solve(input: &str) -> u64 {
    find_no_valid(input, 25)
}

fn find_no_valid(input: &str, preamble: usize) -> u64 {
    let numbers: Vec<u64> = input.trim().lines().map(|n| n.parse().unwrap()).collect();
    for (index, sum) in numbers.iter().enumerate().skip(preamble) {
        let sums: HashSet<u64> = cart_product(
            numbers[(index - preamble)..index].iter(),
            numbers[(index - preamble)..index].iter(),
        )
        .map(|(a, b)| a + b)
        .collect();

        if !sums.contains(sum) {
            return *sum;
        }
    }

    panic!("they are all valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(find_no_valid(input, 5), 127);
    }
}

common::read_main!();
