use std::{
    cmp,
    collections::{BTreeSet, BinaryHeap, HashMap},
};

use common::read_main;

fn solve(input: &str) -> u16 {
    let valves: HashMap<_, _> = input
        .trim()
        .lines()
        .map(|reading| {
            let reading = reading.strip_prefix("Valve ").unwrap();
            let (name, reading) = reading.split_once(' ').unwrap();
            let reading = reading.strip_prefix("has flow rate=").unwrap();
            let (rate, reading) = reading.split_once(';').unwrap();
            let reading = reading
                .strip_prefix(" tunnels lead to valves ")
                .or_else(|| reading.strip_prefix(" tunnel leads to valve "))
                .unwrap();
            (
                name,
                Valve {
                    name,
                    rate: rate.parse().unwrap(),
                    tunnels: reading.split(", ").collect(),
                },
            )
        })
        .collect();

    let unopened: BTreeSet<_> = valves.values().filter(|valves| valves.rate > 0).collect();

    let theoretical_max = calculate_potential(&unopened, 26, [false, false]);
    let start = valves.get("AA").unwrap();

    let node = Node {
        cost: 0,
        time_left: 26,
        positions: [start, start],
        unopened,
    };

    let mut frontier = BinaryHeap::from_iter([cmp::Reverse(node.clone())]);
    let mut explored: HashMap<_, _> =
        HashMap::from_iter([((node.positions, node.unopened.clone()), node.cost)]);

    while let Some(cmp::Reverse(node)) = frontier.pop() {
        if node.unopened.is_empty() {
            return theoretical_max - node.cost;
        };

        let Some(time_left) = node.time_left.checked_sub(1) else {
            return theoretical_max - node.cost;
        };

        let at_opened = {
            if node.positions[0] == node.positions[1] {
                [node.unopened.contains(node.positions[0]), false]
            } else {
                [
                    node.unopened.contains(node.positions[0]),
                    node.unopened.contains(node.positions[1]),
                ]
            }
        };

        let max_cost = node.cost + calculate_potential(&node.unopened, node.time_left, at_opened);

        if at_opened[0] {
            let mut unopened = node.unopened.clone();
            unopened.remove(node.positions[0]);
            let max_cost = max_cost - node.positions[0].rate * time_left as u16;

            if at_opened[1] {
                let mut unopened = unopened.clone();
                unopened.remove(node.positions[1]);
                let max_cost = max_cost - node.positions[1].rate * time_left as u16;
                let cost = max_cost - calculate_potential(&unopened, time_left, [false, false]);

                let key = (node.positions, unopened.clone());
                match explored.get(&key) {
                    Some(old_cost) if *old_cost <= cost => {}
                    _ => {
                        explored.insert(key, cost);
                        let next = Node {
                            cost,
                            time_left,
                            positions: node.positions,
                            unopened,
                        };

                        frontier.push(cmp::Reverse(next));
                    }
                }
            }

            let next = node.positions[1].tunnels.iter().filter_map(|tunnel| {
                let valve = valves.get(tunnel).unwrap();
                let unopened = unopened.clone();
                let cost = max_cost
                    - calculate_potential(&unopened, time_left, [false, unopened.contains(valve)]);
                let mut positions = [node.positions[0], valve];
                positions.sort(); // sort to normalize
                let key = (positions, unopened.clone());
                match explored.get(&key) {
                    Some(old_cost) if *old_cost <= cost => None,
                    _ => {
                        explored.insert(key, cost);
                        Some(Node {
                            cost,
                            time_left,
                            positions,
                            unopened,
                        })
                    }
                }
            });

            frontier.extend(next.map(cmp::Reverse));
        }

        if at_opened[1] {
            let mut unopened = node.unopened.clone();
            unopened.remove(node.positions[1]);
            let max_cost = max_cost - node.positions[1].rate * time_left as u16;

            let next = node.positions[0].tunnels.iter().filter_map(|tunnel| {
                let valve = valves.get(tunnel).unwrap();
                let unopened = unopened.clone();
                let cost = max_cost
                    - calculate_potential(&unopened, time_left, [unopened.contains(valve), false]);
                let mut positions = [valve, node.positions[1]];
                positions.sort(); // sort to normalize
                let key = (positions, unopened.clone());
                match explored.get(&key) {
                    Some(old_cost) if *old_cost <= cost => None,
                    _ => {
                        explored.insert(key, cost);
                        Some(Node {
                            cost,
                            time_left,
                            positions,
                            unopened,
                        })
                    }
                }
            });

            frontier.extend(next.map(cmp::Reverse));
        }

        let next = node.positions[0]
            .tunnels
            .iter()
            .flat_map(|&tunnel_0| {
                node.positions[1]
                    .tunnels
                    .iter()
                    .map(move |&tunnel_1| [tunnel_0, tunnel_1])
            })
            .filter_map(|tunnels| {
                let mut positions = [
                    valves.get(tunnels[0]).unwrap(),
                    valves.get(tunnels[1]).unwrap(),
                ];
                positions.sort(); // sort to normalize

                let unopened = node.unopened.clone();
                let at_opened = {
                    if positions[0] == positions[1] {
                        [unopened.contains(positions[0]), false]
                    } else {
                        [
                            unopened.contains(positions[0]),
                            unopened.contains(positions[1]),
                        ]
                    }
                };
                let cost = max_cost - calculate_potential(&unopened, time_left, at_opened);
                let key = (positions, unopened.clone());
                match explored.get(&key) {
                    Some(old_cost) if *old_cost <= cost => None,
                    _ => {
                        explored.insert(key, cost);
                        Some(Node {
                            cost,
                            time_left,
                            positions,
                            unopened,
                        })
                    }
                }
            });

        frontier.extend(next.map(cmp::Reverse));
    }

    unreachable!()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Valve<'s> {
    rate: u16,
    name: &'s str,
    tunnels: Vec<&'s str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node<'s, 'v> {
    // how away are we from the perfect fit
    cost: u16,
    // if two are in equal cost grab first the one that's almost done
    time_left: u8,

    // for state tracking; the sorting is 🤷
    positions: [&'v Valve<'s>; 2],
    unopened: BTreeSet<&'v Valve<'s>>,
}

fn calculate_potential(
    unopened: &BTreeSet<&Valve>,
    mut time_left: u8,
    mut at_opened: [bool; 2],
) -> u16 {
    let mut potential = 0;
    let mut unopened = unopened.iter().rev();
    loop {
        let Some(after_opening) = time_left.checked_sub(1) else { break };
        if at_opened[0] {
            let Some(valve) = unopened.next() else {break};
            potential += valve.rate * after_opening as u16;
        }
        if at_opened[1] {
            let Some(valve) = unopened.next() else {break};
            potential += valve.rate * after_opening as u16;
        }
        at_opened[0] = !at_opened[0];
        at_opened[1] = !at_opened[1];
        time_left = after_opening;
    }
    potential
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
        assert_eq!(solve(input), 1707);
    }
}

read_main!();
