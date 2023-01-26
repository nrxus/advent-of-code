use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    let mut positions: HashSet<_> = input
        .trim()
        .chars()
        .scan((0, 0), |pos, c| {
            match c {
                '>' => pos.0 += 1,
                '<' => pos.0 -= 1,
                '^' => pos.1 -= 1,
                'v' => pos.1 += 1,
                c => panic!("unexpected: {c}"),
            };
            Some(*pos)
        })
        .collect();

    positions.insert((0, 0));

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(">"), 2);
        assert_eq!(solve("^>v<"), 4);
        assert_eq!(solve("^v^v^v^v^v"), 2);
    }
}

read_main!();
