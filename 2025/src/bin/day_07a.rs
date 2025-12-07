use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> u64 {
    let input = input.trim();
    let height = input.lines().count();

    let mut beams = VecDeque::new();

    let splitters: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, c)| ((x, y), c)))
        .inspect(|(pos, c)| {
            if *c == b'S' {
                beams.push_front(*pos);
            }
        })
        .filter_map(|(pos, c)| if c == b'^' { Some(pos) } else { None })
        .collect();

    let mut explored = HashSet::new();
    let mut num_splits = 0;

    while let Some((x, y)) = beams.pop_back() {
        let new_beam = (x, y + 1);
        if new_beam.1 >= height {
            continue;
        }
        if splitters.contains(&new_beam) {
            num_splits += 1;
            let new_beams = [(new_beam.0 - 1, new_beam.1), (new_beam.0 + 1, new_beam.1)];
            new_beams.into_iter().for_each(|new_beam| {
                if explored.insert(new_beam) {
                    beams.push_front(new_beam);
                }
            });
        } else if explored.insert(new_beam) {
            beams.push_front(new_beam);
        }
    }

    num_splits
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        ),
        21
    );
}
