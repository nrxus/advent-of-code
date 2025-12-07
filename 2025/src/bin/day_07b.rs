use std::collections::{hash_map, HashMap, HashSet, VecDeque};

fn solve(input: &str) -> u64 {
    let input = input.trim();
    let height = input.lines().count();

    let mut beams = VecDeque::new();
    let mut explored = HashMap::new();

    let splitters: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, c)| ((x, y), c)))
        .inspect(|(pos, c)| {
            if *c == b'S' {
                beams.push_front(*pos);
                explored.insert(*pos, 1);
            }
        })
        .filter_map(|(pos, c)| if c == b'^' { Some(pos) } else { None })
        .collect();

    let mut num_universes = 0;

    while let Some(beam) = beams.pop_back() {
        let beam_universes = explored.remove(&beam).unwrap();
        let new_beam = (beam.0, beam.1 + 1);
        if new_beam.1 >= height {
            num_universes += beam_universes;
            continue;
        }
        let mut add_beam = |new_beam| match explored.entry(new_beam) {
            hash_map::Entry::Occupied(o) => *o.into_mut() += beam_universes,
            hash_map::Entry::Vacant(v) => {
                v.insert(beam_universes);
                beams.push_front(new_beam);
            }
        };
        if splitters.contains(&new_beam) {
            let new_beams = [(new_beam.0 - 1, new_beam.1), (new_beam.0 + 1, new_beam.1)];
            new_beams.into_iter().for_each(add_beam);
        } else {
            add_beam(new_beam);
        }
    }

    num_universes
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
        40
    );
}
