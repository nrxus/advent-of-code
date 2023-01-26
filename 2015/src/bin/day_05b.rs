use std::collections::{hash_map, HashMap};

use common::read_main;

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let line = line.as_bytes();

            if !line.windows(3).any(|pair| pair[0] == pair[2]) {
                return false;
            }

            let mut pairs = HashMap::new();
            for (i, pair) in line.windows(2).enumerate() {
                match pairs.entry(pair) {
                    hash_map::Entry::Occupied(o) => {
                        if *o.get() < i - 1 {
                            return true;
                        }
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert(i);
                    }
                }
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";
        assert_eq!(solve(input), 2);
    }
}

read_main!();
