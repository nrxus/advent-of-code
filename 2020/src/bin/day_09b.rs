use common::extensions::cart_product;
use std::collections::HashSet;

fn solve(input: &str) -> u64 {
    encryption_weakness(input, 25)
}

fn encryption_weakness(input: &str, preamble: usize) -> u64 {
    let numbers: Vec<u64> = input.trim().lines().map(|n| n.parse().unwrap()).collect();
    let not_valid = find_no_valid(&numbers, preamble);

    for (index, number) in numbers[..numbers.len() - 1]
        .iter()
        .enumerate()
        .filter(|(_, n)| **n < not_valid)
    {
        // guaranteeed to be at least 1 by filter above
        let goal = not_valid - number;

        let (last_i, sum) = numbers[index + 1..]
            .iter()
            .scan(0, |sum, number| {
                *sum += number;
                Some(*sum)
            })
            .take_while(|sum| *sum <= goal)
            .enumerate()
            .last()
            .unwrap();

        if goal == sum {
            let max = numbers[index..=(index + last_i + 1)].iter().max().unwrap();
            let min = numbers[index..=(index + last_i + 1)].iter().min().unwrap();
            return max + min;
        }
    }

    panic!("did not find encryption_weakness")
}

fn find_no_valid(numbers: &[u64], preamble: usize) -> u64 {
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
        assert_eq!(encryption_weakness(input, 5), 62);
    }
}

common::read_main!();
