use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut positions: HashSet<_> = input
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

    let mut removed = remove_accessible(&mut positions);
    let mut total_removed = removed;
    while removed > 0 {
        removed = remove_accessible(&mut positions);
        total_removed += removed;
    }

    total_removed
}

fn remove_accessible(positions: &mut HashSet<(i32, i32)>) -> usize {
    let op = positions.clone();

    positions
        .extract_if(|&(x, y)| {
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
            .filter(|p| op.contains(p))
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
        43
    );
}
