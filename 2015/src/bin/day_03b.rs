use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let santa = input.chars().step_by(2).coordinates();
    let robot_santa = input.chars().skip(1).step_by(2).coordinates();

    let mut grid = HashSet::new();
    grid.insert((0, 0));
    grid.extend(santa);
    grid.extend(robot_santa);
    grid.len()
}

trait DirectionIterExt {
    fn coordinates(self) -> HashSet<(i32, i32)>;
}

impl<I: Iterator<Item = char>> DirectionIterExt for I {
    fn coordinates(self) -> HashSet<(i32, i32)> {
        self.map(|c| match c {
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
        .collect::<HashSet<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(solve("^v"), 3);
    }

    #[test]
    fn test_many() {
        assert_eq!(solve("^>v<"), 3);
    }

    #[test]
    fn repeated() {
        assert_eq!(solve("^v^v^v^v^v"), 11);
    }
}

common::read_main!();
