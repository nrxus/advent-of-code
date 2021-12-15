use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
    fmt::{self, Formatter, Write},
    str::FromStr,
};

fn solve(input: &str) -> u32 {
    let map = RiskMap::from_str(input).unwrap();
    map.lowest_risk()
}

#[derive(Debug)]
pub struct RiskMap {
    levels: Vec<u32>,
    width: usize,
}

impl RiskMap {
    pub fn lowest_risk(&self) -> u32 {
        if self.levels.is_empty() {
            return 0;
        }

        let goal_index = self.levels.len() - 1;
        let mut frontier: BinaryHeap<cmp::Reverse<Node>> = BinaryHeap::new();
        frontier.push(cmp::Reverse(Node { index: 0, risk: 0 }));
        let mut explored: HashSet<usize> = HashSet::new();

        while let Some(cmp::Reverse(node)) = frontier.pop() {
            if node.index == goal_index {
                return node.risk;
            }

            if !explored.insert(node.index) {
                continue;
            }

            frontier.extend(
                self.neighbors(node.index)
                    .map(|n| self.make_node(n, node.risk)),
            );
        }

        unreachable!()
    }

    fn make_node(&self, index: usize, current_risk: u32) -> cmp::Reverse<Node> {
        cmp::Reverse(Node {
            index,
            risk: current_risk + self.levels[index],
        })
    }

    fn neighbors(&self, index: usize) -> impl Iterator<Item = usize> {
        let left = index
            .checked_sub(1)
            .filter(|left| left % self.width < index % self.width);

        let right = Some(index + 1).filter(|right| right % self.width > index % self.width);

        let up = Some(index + self.width).filter(|up| *up < self.levels.len());

        let down = index.checked_sub(self.width);

        [left, right, up, down].into_iter().flatten()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    risk: u32,
    // index has to come *after* risk so the PartialOrd derive prioritizes it
    index: usize,
}

impl FromStr for RiskMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        let original_len = s.lines().next().map(|l| l.len()).unwrap_or_default();
        let total_len = original_len * 5;
        let cycle_digit = |digit, repeated| {
            let mut d = digit + repeated;
            if d > 9 {
                d -= 9;
            }
            assert!(d <= 9);

            d
        };

        let levels: Vec<_> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .cycle()
                    .take(total_len)
                    .enumerate()
                    .map(|(i, d)| {
                        let repeated = (i / original_len) as u32;
                        cycle_digit(d, repeated)
                    })
            })
            .cycle()
            .take(total_len)
            .enumerate()
            .flat_map(|(i, line)| {
                let repeated = (i / original_len) as u32;
                line.map(move |d| cycle_digit(d, repeated))
            })
            .collect();

        Ok(Self {
            levels,
            width: original_len * 5,
        })
    }
}

impl fmt::Display for RiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.levels.chunks(self.width).try_for_each(|line| {
            line.iter()
                .try_for_each(|&d| f.write_char(char::from_digit(d, 10).unwrap()))?;

            f.write_str("\n")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(solve(input), 315);
    }
}

common::read_main!();
