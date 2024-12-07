use std::collections::HashSet;

fn solve(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (test, numbers) = line.split_once(':').unwrap();
            let test: u64 = test.parse().unwrap();
            let numbers: Vec<u64> = numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mut states: HashSet<u64> = HashSet::from_iter([numbers[0]]);
            for number in &numbers[1..] {
                states = states
                    .into_iter()
                    .flat_map(|n| [n + number, n * number])
                    .filter(|n| *n <= test)
                    .collect();
            }

            Some(test).filter(|test| states.contains(test))
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
        ),
        3749
    );
}
