use std::collections::{HashSet, VecDeque};

use common::read_main;

fn solve(input: &str) -> usize {
    let input = input.trim();
    let num_cols = input.lines().next().unwrap().len();

    let mut explored: HashSet<_> = HashSet::new();
    let mut end = None;

    let data = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .enumerate()
        .map(|(i, &e)| {
            if e == b'S' {
                explored.insert(i);
                b'a'
            } else if e == b'E' {
                assert!(end.replace(i).is_none());
                b'z'
            } else {
                if e == b'a' {
                    explored.insert(i);
                }
                e
            }
        })
        .collect();

    let end = end.unwrap();

    let mut frontier: VecDeque<_> = explored.iter().map(|index| (0, *index)).collect();

    let map = Rectangle { data, num_cols };

    while let Some((cost, index)) = frontier.pop_front() {
        if index == end {
            return cost;
        }

        let neighbors = map.neighbors(index).filter_map(|ni| {
            if explored.contains(&ni) {
                return None;
            }
            if map.data[ni] > (map.data[index] + 1) {
                return None;
            }
            explored.insert(ni);
            Some((cost + 1, ni))
        });

        frontier.extend(neighbors);
    }

    panic!("failed to find path to node at index: {end}")
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
        assert_eq!(solve(input), 29);
    }
}

read_main!();
