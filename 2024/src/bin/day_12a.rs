use std::collections::{hash_map, HashMap};

fn solve(input: &str) -> usize {
    let map: HashMap<(isize, isize), u8> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, b)| ((x as isize, y as isize), b))
        })
        .collect();

    const DELTAS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut cost = 0;
    let mut traveled_map = map.clone();

    while let Some((pos, plant)) = pop_any(&mut traveled_map) {
        let mut frontier = vec![pos];
        let mut area = 0;
        let mut perimeter = 0;

        while let Some((x, y)) = frontier.pop() {
            area += 1;
            perimeter += DELTAS
                .into_iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|pos| map.get(pos) != Some(&plant))
                .count();

            let neighbors = DELTAS
                .into_iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter_map(|pos| {
                    let hash_map::Entry::Occupied(entry) = traveled_map.entry(pos) else {
                        return None;
                    };
                    if *entry.get() != plant {
                        return None;
                    };
                    let (pos, _) = entry.remove_entry();
                    Some(pos)
                });

            frontier.extend(neighbors);
        }

        cost += area * perimeter;
    }

    cost
}

fn pop_any<K: Eq + std::hash::Hash + Clone, V>(map: &mut HashMap<K, V>) -> Option<(K, V)> {
    let next = map.keys().next()?.clone();
    map.remove_entry(&next)
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"AAAA
BBCD
BBCC
EEEC
"
        ),
        140
    );
}

#[test]
fn example_two() {
    assert_eq!(
        solve(
            r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
        ),
        772
    );
}

#[test]
fn example_three() {
    assert_eq!(
        solve(
            r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
        ),
        1930
    );
}
