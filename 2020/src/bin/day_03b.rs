use std::collections::HashSet;

fn solve(map: &str) -> usize {
    let map = map.trim();

    let length = map.lines().count();
    let width = map.lines().nth(0).map(|l| l.chars().count()).unwrap();

    let trees: HashSet<_> = map
        .lines()
        .enumerate()
        .flat_map(|(y, horizontal)| {
            horizontal
                .chars()
                .enumerate()
                .filter_map(
                    move |(x, terrain)| {
                        if terrain == '#' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
        })
        .collect();

    let cases: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    cases
        .iter()
        .map(|slope| {
            std::iter::successors(Some((0, 0)), |p| {
                if p.1 < (length - slope.1) {
                    Some(((p.0 + slope.0) % width, p.1 + slope.1))
                } else {
                    None
                }
            })
            .filter(|p| trees.contains(p))
            .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!(solve(input), 336);
    }
}

common::read_main!();
