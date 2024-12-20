use std::{
    cmp,
    collections::{hash_map, BinaryHeap, HashMap, HashSet, VecDeque},
};

fn solve(input: &str) -> usize {
    let mut start = None;
    let mut end = None;

    let walls: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b != b'.')
                .map(move |(x, b)| ((x, y), b))
        })
        .filter_map(|(pos, c)| match c {
            b'#' => Some(pos),
            b'S' => {
                assert!(start.replace(pos).is_none());
                None
            }
            b'E' => {
                assert!(end.replace(pos).is_none());
                None
            }
            _ => unreachable!(),
        })
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    fn calculate_dist(a: Coord, b: Coord) -> u32 {
        (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
    }

    let cost_graph = get_normal_cost(end, &walls);
    let non_cheat_cost = cost_graph.get(&start).unwrap();
    let max_cost = non_cheat_cost - MIN_SAVED;
    let node = (start, Cheat::NotCheated);
    let mut frontier =
        BinaryHeap::from_iter([(cmp::Reverse((calculate_dist(start, end), 0)), node)]);
    let mut cheats: HashSet<(Coord, Coord)> = HashSet::new();
    let mut explored: HashSet<(Coord, Cheat)> = HashSet::from_iter([node]);

    while let Some((cmp::Reverse((_, cost)), (pos, cheat))) = frontier.pop() {
        if pos == end {
            let Cheat::CheatedStarted { started, .. } = cheat else {
                unreachable!();
            };
            if cost <= max_cost {
                cheats.insert((started, pos));
            }
        }

        let cost = cost + 1;
        // any future movement cost will be too high
        if cost > max_cost {
            continue;
        }

        let (x, y) = pos;
        let left = x.checked_sub(1);
        let right = Some(x + 1);
        let up = y.checked_sub(1);
        let down = Some(y + 1);
        let neighbors = [
            up.map(|y| (x, y)),
            down.map(|y| (x, y)),
            right.map(|x| (x, y)),
            left.map(|x| (x, y)),
        ]
        .into_iter()
        .flatten()
        .flat_map(|next| match cheat {
            Cheat::NotCheated => {
                let start_cheating = Cheat::CheatedStarted {
                    started: pos,
                    cheat_ps: 1,
                };
                if walls.contains(&next) {
                    vec![(next, start_cheating)]
                } else {
                    vec![(next, start_cheating), (next, Cheat::NotCheated)]
                }
            }
            Cheat::CheatedStarted { started, cheat_ps } => {
                let cheat_ps = cheat_ps + 1;

                if !walls.contains(&next) {
                    if let Some(remaining_cost) = cost_graph.get(&next) {
                        let total_cost = cost + remaining_cost;
                        if total_cost <= max_cost {
                            cheats.insert((started, next));
                        }
                    }
                }

                if cheat_ps >= 20 {
                    vec![]
                } else {
                    vec![(next, Cheat::CheatedStarted { started, cheat_ps })]
                }
            }
        })
        .filter(|node| explored.insert(*node))
        .filter_map(|node| {
            let estimated_cost = cost + calculate_dist(node.0, end);
            if estimated_cost > max_cost {
                None
            } else {
                Some((cmp::Reverse((cost, cost)), node))
            }
        });

        frontier.extend(neighbors);
    }

    cheats.len()
}

fn get_normal_cost(end: Coord, walls: &HashSet<Coord>) -> HashMap<Coord, u32> {
    let mut frontier = VecDeque::from_iter([(end, 0)]);
    let mut graph: HashMap<Coord, u32> = HashMap::from_iter([(end, 0)]);

    while let Some((pos, cost)) = frontier.pop_front() {
        let (x, y) = pos;
        let left = x.checked_sub(1);
        let right = Some(x + 1);
        let up = y.checked_sub(1);
        let down = Some(y + 1);
        let neighbors = [
            up.map(|y| (x, y)),
            down.map(|y| (x, y)),
            right.map(|x| (x, y)),
            left.map(|x| (x, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|next| !walls.contains(next))
        .filter_map(|node| {
            let cost = cost + 1;
            let hash_map::Entry::Vacant(v) = graph.entry(node) else {
                return None;
            };
            v.insert(cost);
            Some((node, cost))
        });

        frontier.extend(neighbors);
    }

    graph
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Cheat {
    NotCheated,
    CheatedStarted { started: Coord, cheat_ps: u8 },
}

type Coord = (usize, usize);

#[cfg(test)]
const MIN_SAVED: u32 = 72;

#[cfg(not(test))]
const MIN_SAVED: u32 = 100;

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        ),
        29
    );
}
