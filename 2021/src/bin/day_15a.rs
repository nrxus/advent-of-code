use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
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

#[derive(Eq)]
pub struct Node {
    index: usize,
    risk: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // only consider risk for odering
        self.risk.cmp(&other.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        // only consider risk for odering
        self.risk == other.risk
    }
}

impl FromStr for RiskMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        let width = s.lines().next().map(|l| l.len()).unwrap_or_default();
        let levels: Vec<_> = input
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| c.to_digit(10))
            .collect::<Option<_>>()
            .ok_or_else(|| "character in map was not a digit".to_owned())?;

        Ok(Self { levels, width })
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
        assert_eq!(solve(input), 40);
    }
}

common::read_main!();
