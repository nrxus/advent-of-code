use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

use common::read_main;

fn solve(input: &str) -> usize {
    let input = input.trim();
    let num_cols = input.lines().next().unwrap().len();
    let mut start = None;
    let mut end = None;

    let data = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .enumerate()
        .map(|(i, &e)| {
            if e == b'S' {
                assert!(start.replace(i).is_none());
                b'a'
            } else if e == b'E' {
                assert!(end.replace(i).is_none());
                b'z'
            } else {
                e
            }
        })
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    let mut explored: HashSet<usize> = HashSet::from_iter([start]);
    let map = Rectangle { data, num_cols };
    let mut frontier = BinaryHeap::from_iter([cmp::Reverse(Node {
        estimated_cost: std::cmp::max(
            map.distance(start, end),
            (map.data[end] - map.data[start]) as usize,
        ),
        cost_so_far: 0,
        index: start,
    })]);

    while let Some(cmp::Reverse(Node {
        cost_so_far, index, ..
    })) = frontier.pop()
    {
        if index == end {
            return cost_so_far;
        }

        let neighbors = map.neighbors(index).filter_map(|ni| {
            if explored.contains(&ni) {
                return None;
            }
            if map.data[ni] > (map.data[index] + 1) {
                return None;
            }
            explored.insert(ni);
            Some(cmp::Reverse(Node {
                estimated_cost: std::cmp::max(
                    map.distance(ni, end),
                    (map.data[end] - map.data[ni]) as usize,
                ) + cost_so_far
                    + 1,
                cost_so_far: cost_so_far + 1,
                index: ni,
            }))
        });

        frontier.extend(neighbors);
    }

    panic!("failed to find path to node at index: {end}")
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    estimated_cost: usize,
    cost_so_far: usize,
    index: usize,
}

#[derive(Debug)]
struct Rectangle<T> {
    data: Vec<T>,
    num_cols: usize,
}

impl<T> Rectangle<T> {
    pub fn neighbors(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        let left = index
            .checked_sub(1)
            .filter(|i| i % self.num_cols != (self.num_cols - 1));
        let right = Some(index + 1).filter(|i| i % self.num_cols != 0);
        let up = index.checked_sub(self.num_cols);
        let down = Some(index + self.num_cols).filter(|i| *i < self.data.len());
        left.into_iter()
            .chain(right.into_iter())
            .chain(up.into_iter())
            .chain(down.into_iter())
    }

    pub fn distance(&self, a: usize, b: usize) -> usize {
        let a = (a / self.num_cols, a % self.num_cols);
        let b = (b / self.num_cols, b % self.num_cols);
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        assert_eq!(solve(input), 31);
    }
}

read_main!();
