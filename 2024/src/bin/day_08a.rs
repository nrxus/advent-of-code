use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().map(|l| l.len()).unwrap();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, c)| *c != b'.')
                .map(move |(x, c)| (c, (x, y)))
        })
        .fold(HashMap::new(), |mut antennas, (antenna, pos)| {
            antennas.entry(antenna).or_insert(vec![]).push(pos);
            antennas
        });

    let antinodes: HashSet<_> = antennas
        .values()
        .flat_map(|positions| {
            positions[..positions.len() - 1]
                .iter()
                .enumerate()
                .flat_map(|(i, &a)| positions[i + 1..].iter().map(move |&b| [a, b]))
                .flat_map(|[a, b]| {
                    let (ax, bx) = if a.0 < b.0 {
                        let x_delta = b.0 - a.0;
                        let ax = a.0.checked_sub(x_delta);
                        let bx = Some(b.0 + x_delta).filter(|x| *x < width);

                        (ax, bx)
                    } else {
                        let x_delta = a.0 - b.0;
                        let bx = b.0.checked_sub(x_delta);
                        let ax = Some(a.0 + x_delta).filter(|x| *x < width);

                        (ax, bx)
                    };

                    let (ay, by) = if a.1 < b.1 {
                        let y_delta = b.1 - a.1;
                        let ay = a.1.checked_sub(y_delta);
                        let by = Some(b.1 + y_delta).filter(|y| *y < height);

                        (ay, by)
                    } else {
                        let y_delta = a.1 - b.1;
                        let by = b.1.checked_sub(y_delta);
                        let ay = Some(a.1 + y_delta).filter(|y| *y < height);

                        (ay, by)
                    };

                    [ax.zip(ay), bx.zip(by)].into_iter().flatten()
                })
        })
        .collect();

    antinodes.len()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
        ),
        14
    );
}
