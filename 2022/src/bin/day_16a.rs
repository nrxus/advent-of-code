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

    let theoretical_max = calculate_potential(&unopened, 30);

    let node = Node {
        cost: 0,
        time_left: 30,
        valve: valves.get("AA").unwrap(),
        unopened,
    };

    let mut frontier = BinaryHeap::from_iter([cmp::Reverse(node.clone())]);
    let mut explored: HashMap<(&Valve, BTreeSet<&Valve>), u16> =
        HashMap::from_iter([((node.valve, node.unopened.clone()), node.cost)]);

    while let Some(cmp::Reverse(node)) = frontier.pop() {
        if node.unopened.is_empty() {
            return theoretical_max - node.cost;
        };

        let max_cost: u16;

        let Some(time_left) = node.time_left.checked_sub(1) else {
            return theoretical_max - node.cost;
        };

        if node.unopened.contains(node.valve) {
            // time_left + 1 to simulate moving to a valve
            let potential = calculate_potential(&node.unopened, node.time_left + 1);
            max_cost = node.cost + potential;
            let mut unopened = node.unopened.clone();
            unopened.remove(node.valve);

            let remaining_potential = calculate_potential(&unopened, time_left);
            let cost = max_cost - remaining_potential - (node.valve.rate * time_left as u16);
            let key = (node.valve, unopened.clone());
            match explored.get(&key) {
                Some(old_cost) if *old_cost <= cost => {}
                _ => {
                    explored.insert(key, cost);
                    let next = Node {
                        cost,
                        time_left,
                        valve: node.valve,
                        unopened,
                    };

                    frontier.push(cmp::Reverse(next));
                }
            }
        } else {
            let potential = calculate_potential(&node.unopened, node.time_left);
            max_cost = node.cost + potential;
        }

        let next = node.valve.tunnels.iter().filter_map(|tunnel| {
            let valve = valves.get(tunnel).unwrap();
            let unopened = node.unopened.clone();
            let cost = max_cost
                - if node.unopened.contains(valve) {
                    calculate_potential(&node.unopened, time_left + 1)
                } else {
                    calculate_potential(&node.unopened, time_left)
                };
            let key = (valve, unopened.clone());
            match explored.get(&key) {
                Some(old_cost) if *old_cost <= cost => None,
                _ => {
                    explored.insert(key, cost);
                    Some(Node {
                        cost,
                        time_left,
                        valve,
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
    valve: &'v Valve<'s>,
    unopened: BTreeSet<&'v Valve<'s>>,
}

fn calculate_potential(unopened: &BTreeSet<&Valve>, mut time_left: u8) -> u16 {
    let mut potential = 0;
    for valve in unopened.iter().rev() {
        let Some(after_opening) = time_left.checked_sub(2) else { break };
        potential += valve.rate * after_opening as u16;
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
        assert_eq!(solve(input), 1651);
    }
}

read_main!();
