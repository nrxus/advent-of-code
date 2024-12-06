use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let mut start = None;
    let map: HashMap<_, _> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .inspect(|(pos, c)| {
            if *c == b'^' {
                if start.replace(*pos).is_some() {
                    panic!("double start");
                }
            }
        })
        .collect();

    let start = start.expect("no start");
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter([start]);
    let mut current = start;
    let mut direction = (0, -1);
    let mut next = (current.0 + direction.0, current.1 + direction.1);

    while let Some(&c) = map.get(&next) {
        if c == b'#' {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };
        } else {
            visited.insert(next);
            current = next;
        }

        next = (current.0 + direction.0, current.1 + direction.1);
    }

    visited.remove(&start);

    visited
        .into_iter()
        .filter(|&new_obstacle| {
            let mut direction = (0, -1);
            let mut visited: HashSet<((isize, isize), (isize, isize))> =
                HashSet::from_iter([(start, direction)]);
            let mut current = start;

            let mut next = (current.0 + direction.0, current.1 + direction.1);

            while let Some(&c) = map.get(&next) {
                if c == b'#' || next == new_obstacle {
                    direction = match direction {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => unreachable!(),
                    };
                } else {
                    if !visited.insert((next, direction)) {
                        return true;
                    }
                    current = next;
                }

                next = (current.0 + direction.0, current.1 + direction.1);
            }

            false
        })
        .count()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
        ),
        6
    );
}
