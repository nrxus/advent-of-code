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
                    let dx = a.0.abs_diff(b.0);
                    let dy = a.1.abs_diff(b.1);

                    let next = |subx, suby| {
                        move |(x, y): &(usize, usize)| {
                            let x = if subx {
                                x.checked_sub(dx)
                            } else {
                                Some(x + dx).filter(|x| *x < width)
                            };
                            let y = if suby {
                                y.checked_sub(dy)
                            } else {
                                Some(y + dy).filter(|y| *y < height)
                            };
                            x.zip(y)
                        }
                    };

                    let first = std::iter::successors(Some(a), next(a.0 < b.0, a.1 < b.1));
                    let second = std::iter::successors(Some(b), next(a.0 > b.0, a.1 > b.1));

                    first.chain(second)
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
        34
    );
}
