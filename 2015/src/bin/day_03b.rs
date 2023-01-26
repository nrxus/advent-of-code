use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    let mut positions: HashSet<_> = input
        .trim()
        .chars()
        .enumerate()
        .scan(((0, 0), (0, 0)), |(santa, robot), (i, c)| {
            let changed = if i % 2 == 0 { santa } else { robot };

            match c {
                '>' => changed.0 += 1,
                '<' => changed.0 -= 1,
                '^' => changed.1 -= 1,
                'v' => changed.1 += 1,
                c => panic!("unexpected: {c}"),
            };

            Some(*changed)
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
        assert_eq!(solve("^v"), 3);
        assert_eq!(solve("^>v<"), 3);
        assert_eq!(solve("^v^v^v^v^v"), 11);
    }
}

read_main!();
