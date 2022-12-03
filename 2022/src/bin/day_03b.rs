use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> u32 {
    // if iterator::array_chunks was stabilized we wouldn't need to
    // collect these lines:
    // https://github.com/rust-lang/rust/issues/100450 😔

    let lines: Vec<_> = input.trim().lines().collect();
    lines
        .chunks(3)
        .map(|group| {
            let remainder = group
                .iter()
                .map(|line| line.bytes().collect::<HashSet<_>>())
                .reduce(|acc, line| &acc & &line)
                .unwrap();
            remainder.into_iter().next().unwrap()
        })
        .map(|duplicate| {
            if duplicate.is_ascii_lowercase() {
                (duplicate - b'a' + 1) as u32
            } else {
                (duplicate - b'A' + 27) as u32
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(solve(input), 70);
    }
}

read_main!();
