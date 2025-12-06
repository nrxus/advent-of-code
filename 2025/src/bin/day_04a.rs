use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let positions: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, c)| match c {
                b'@' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .collect();

    positions
        .iter()
        .filter(|&&(x, y)| {
            [
                (x, y - 1),
                (x, y + 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
            ]
            .into_iter()
            .filter(|p| positions.contains(p))
            .take(4)
            .count()
                < 4
        })
        .count()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        ),
        13
    );
}
