#![feature(never_type)]

use std::{
    collections::{HashMap, HashSet},
    str,
};

fn solve(input: &str) -> String {
    let graph: [Option<HashSet<u8>>; 26] = Default::default();
    let mut graph = input
        .lines()
        .map(|l| l.parse::<Dependency>().unwrap())
        .fold(graph, |mut graph, dep| {
            graph[(dep.blocker - b'A') as usize].get_or_insert_with(Default::default);
            graph[(dep.dependent - b'A') as usize]
                .get_or_insert_with(HashSet::default)
                .insert(dep.blocker);
            graph
        });

    let mut graph: HashMap<_, _> = graph
        .iter_mut()
        .enumerate()
        .filter_map(|(i, v)| v.as_mut().map(|v| (i as u8 + b'A', v)))
        .collect();

    let mut route = "".to_owned();

    while !graph.is_empty() {
        let (&val, _) = graph
            .iter()
            .filter(|(_, blockers)| blockers.is_empty())
            .min_by_key(|(&i, _)| i)
            .unwrap();
        graph.remove(&val);
        graph.values_mut().for_each(|blockers| {
            blockers.remove(&val);
        });
        route.push(val as char)
    }

    route
}

struct Dependency {
    blocker: u8,
    dependent: u8,
}

impl str::FromStr for Dependency {
    type Err = !;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.as_bytes();

        let blocker = input[5];
        let dependent = input[36];

        Ok(Dependency { blocker, dependent })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        assert_eq!(solve(input), "CABDFE".to_owned());
    }
}

common::bootstrap!(7);
