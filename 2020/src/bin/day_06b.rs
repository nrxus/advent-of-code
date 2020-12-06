#![feature(iterator_fold_self)]

fn solve(answers: &str) -> usize {
    answers
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<std::collections::HashSet<_>>())
                .fold_first(|collected, answers| {
                    collected.intersection(&answers).cloned().collect()
                })
                .unwrap()
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
