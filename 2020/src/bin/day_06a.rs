fn solve(answers: &str) -> usize {
    answers
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|person| person.chars())
                .collect::<std::collections::HashSet<_>>()
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
        assert_eq!(solve(input), 11);
    }
}

common::read_main!();
