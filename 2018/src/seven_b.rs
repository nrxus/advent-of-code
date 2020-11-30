use std::{
    collections::{HashMap, HashSet},
    str,
};

fn solve(input: &str) -> u16 {
    duration(input, 5, 60)
}

fn duration(input: &str, num_workers: u8, base_duration: u8) -> u16 {
    let graph: [Option<HashSet<u8>>; 26] = Default::default();
    let mut graph = input
        .trim()
        .lines()
        .map(|l| l.parse::<Dependency>().unwrap())
        .fold(graph, |mut graph, dep| {
            graph[(dep.blocker - b'A') as usize].get_or_insert_with(Default::default);
            graph[(dep.dependent - b'A') as usize]
                .get_or_insert_with(HashSet::default)
                .insert(dep.blocker);
            graph
        });

    let mut unworked: HashMap<_, _> = graph
        .iter_mut()
        .enumerate()
        .filter_map(|(i, v)| v.as_mut().map(|v| (i as u8 + b'A', v)))
        .collect();

    let mut working: HashMap<u8, u16> = HashMap::new();
    let mut duration: u16 = 0;

    while !unworked.is_empty() {
        let free_workers = 1 + num_workers as usize - working.len();
        let mut workable: Vec<_> = unworked
            .iter()
            .filter_map(|(&i, blockers)| if blockers.is_empty() { Some(i) } else { None })
            .collect();
        workable.sort_unstable();
        workable.iter().take(free_workers).for_each(|&i| {
            unworked.remove(&i);
            working.insert(i, u16::from(1 + base_duration + i - b'A'));
        });

        let (_, &work_step) = working
            .iter()
            .min_by_key(|(_, work_left)| *work_left)
            .unwrap();
        working
            .iter_mut()
            .for_each(|(_, work_left)| *work_left -= work_step);
        duration += work_step as u16;
        working.retain(|i, work_left| {
            if *work_left == 0 {
                unworked.values_mut().for_each(|blockers| {
                    blockers.remove(&i);
                });
                false
            } else {
                true
            }
        });
    }

    duration + working.iter().map(|(_, &s)| s).sum::<u16>()
}

struct Dependency {
    blocker: u8,
    dependent: u8,
}

#[derive(Debug)]
enum Never {}

impl str::FromStr for Dependency {
    type Err = Never;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.as_bytes();

        let blocker = input[5];
        let dependent = input[36];

        Ok(Dependency { blocker, dependent })
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(duration(input, 1, 0), 15);
    }
}

// common::read_main!();

common::bootstrap!(7);
