use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut grid = input
        .chars()
        .map(|c| match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("not parseable direction"),
        })
        .scan((0, 0), |location, direction| {
            *location = (location.0 + direction.0, location.1 + direction.1);
            Some(*location)
        })
        .collect::<HashSet<_>>();
    grid.insert((0, 0));
    grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(solve(">"), 2);
    }

    #[test]
    fn test_many() {
        assert_eq!(solve("^>v<"), 4);
    }

    #[test]
    fn repeated() {
        assert_eq!(solve("^v^v^v^v^v"), 2);
    }
}

common::read_main!();
