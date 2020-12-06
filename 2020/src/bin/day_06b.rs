use std::collections::HashSet;

fn solve(answers: &str) -> usize {
    answers
        .trim()
        .split("\n\n")
        .map(|group| {
            let mut group = group
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>());

            let first = group.next().unwrap();

            group
                .fold(first, |collected, answers| &collected & &answers)
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve(input), 6);
    }
}

common::read_main!();
