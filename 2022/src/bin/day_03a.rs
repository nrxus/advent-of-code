use common::read_main;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|rucksack| {
            let len = rucksack.len();
            let first = &rucksack[0..len / 2];
            let second = &rucksack[len / 2..];
            first
                .bytes()
                .find(|b| second.as_bytes().contains(b))
                .unwrap()
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

        assert_eq!(solve(input), 157);
    }
}

read_main!();
