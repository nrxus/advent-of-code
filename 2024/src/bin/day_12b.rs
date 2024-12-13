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
        let mut num_corners = 0;

        while let Some((x, y)) = frontier.pop() {
            area += 1;
            num_corners += {
                let left = map.get(&(x - 1, y)) == Some(&plant);
                let right = map.get(&(x + 1, y)) == Some(&plant);
                let down = map.get(&(x, y + 1)) == Some(&plant);
                let up = map.get(&(x, y - 1)) == Some(&plant);
                let no_left_up = map.get(&(x - 1, y - 1)) != Some(&plant);
                let no_left_down = map.get(&(x - 1, y + 1)) != Some(&plant);
                let no_right_up = map.get(&(x + 1, y - 1)) != Some(&plant);
                let no_right_down = map.get(&(x + 1, y + 1)) != Some(&plant);

                (!left && !down || left && down && no_left_down) as usize
                    + (!left && !up || left && up && no_left_up) as usize
                    + (!right && !down || right && down && no_right_down) as usize
                    + (!right && !up || right && up && no_right_up) as usize
            };

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

        cost += area * num_corners;
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
        80
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
        436
    );
}

#[test]
fn example_e() {
    assert_eq!(
        solve(
            r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
        ),
        236
    );
}

#[test]
fn example_ab() {
    assert_eq!(
        solve(
            r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
        ),
        368
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
        1206
    );
}
