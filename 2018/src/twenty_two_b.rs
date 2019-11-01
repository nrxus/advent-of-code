#![feature(try_trait)]

use common::extensions::{cart_product, AbsDiff};
use regex::Regex;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    num::ParseIntError,
    option::NoneError,
    str::FromStr,
};

fn solve(input: &str) -> u32 {
    Cave::from_str(input).unwrap().time()
}

struct Cave {
    regions: Vec<usize>,
    cols: usize,
    depth: usize,
    target: Node,
}

impl Cave {
    fn time(&self) -> u32 {
        let mut cache = HashMap::new();
        let mut explored = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start = Node {
            position: (0, 0),
            tool: Tool::Torch,
        };
        frontier.push(Step {
            node: start,
            minutes: 0,
            estimate: start.min_minutes(&self.target),
        });

        while let Some(step) = frontier.pop() {
            if step.node == self.target {
                return step.minutes;
            }
            if explored.contains(&step.node) {
                continue;
            }
            explored.insert(step.node);
            let neighbors = self.neighbors(&step, &mut cache);
            let neighbors: Vec<_> = neighbors
                .iter()
                .filter_map(|n| *n)
                .filter(|n| !explored.contains(&n.node))
                .filter(|n| {
                    !frontier
                        .iter()
                        .any(|s| s.node == n.node && s.minutes <= n.minutes)
                })
                .collect();
            frontier.extend(neighbors);
        }

        panic!("goal not reached");
    }

    fn neighbors(
        &self,
        step: &Step,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> [Option<Step>; 5] {
        let mut rooms = [None, None, None, None, None];

        //same room, different tool
        let tools = self.region(step.node.position, cache).tools();
        let node = Node {
            position: step.node.position,
            tool: if tools[0] == step.node.tool {
                tools[1]
            } else {
                tools[0]
            },
        };
        rooms[0] = Some(Step {
            minutes: step.minutes + 7,
            estimate: node.min_minutes(&self.target),
            node,
        });

        //same tool, going up
        rooms[1] = step
            .node
            .position
            .1
            .checked_sub(1)
            .map(|y| (step.node.position.0, y))
            .filter(|&pos| self.region(pos, cache).tools().contains(&step.node.tool))
            .map(|position| Node {
                position,
                tool: step.node.tool,
            })
            .map(|node| Step {
                node,
                estimate: node.min_minutes(&self.target),
                minutes: step.minutes + 1,
            });

        //same tool, going left
        rooms[2] = step
            .node
            .position
            .0
            .checked_sub(1)
            .map(|x| (x, step.node.position.1))
            .filter(|&pos| self.region(pos, cache).tools().contains(&step.node.tool))
            .map(|position| Node {
                position,
                tool: step.node.tool,
            })
            .map(|node| Step {
                node,
                estimate: node.min_minutes(&self.target),
                minutes: step.minutes + 1,
            });

        //same tool, going right
        let right = (step.node.position.0 + 1, step.node.position.1);
        if self.region(right, cache).tools().contains(&step.node.tool) {
            let node = Node {
                position: right,
                tool: step.node.tool,
            };
            rooms[3] = Some(Step {
                node,
                estimate: node.min_minutes(&self.target),
                minutes: step.minutes + 1,
            });
        }

        //same tool, going down
        let down = (step.node.position.0, step.node.position.1 + 1);
        if self.region(down, cache).tools().contains(&step.node.tool) {
            let node = Node {
                position: down,
                tool: step.node.tool,
            };
            rooms[4] = Some(Step {
                node,
                estimate: node.min_minutes(&self.target),
                minutes: step.minutes + 1,
            });
        }

        rooms
    }

    fn region(&self, (x, y): (usize, usize), cache: &mut HashMap<(usize, usize), usize>) -> Region {
        Region::new(if x < self.cols && y * self.cols + x < self.regions.len() {
            self.regions[y * self.cols + x]
        } else {
            self.cached_level((x, y), cache)
        })
    }

    fn cached_level(
        &self,
        (x, y): (usize, usize),
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if x < self.cols && y * self.cols + x < self.regions.len() {
            self.regions[y * self.cols + x]
        } else {
            match cache.get(&(x, y)) {
                None => {
                    let level = Cave::level(x, y, self.depth, || {
                        (
                            self.cached_level((x - 1, y), cache),
                            self.cached_level((x, y - 1), cache),
                        )
                    });
                    cache.insert((x, y), level);
                    level
                }
                Some(level) => *level,
            }
        }
    }

    fn level(x: usize, y: usize, depth: usize, previous: impl FnOnce() -> (usize, usize)) -> usize {
        let geo_index = if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            let (x, y) = previous();
            x * y
        };
        (geo_index + depth) % 20183
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Tool {
    ClimbingGear,
    Neither,
    Torch,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Step {
    minutes: u32,
    estimate: u32,
    node: Node,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
struct Node {
    position: (usize, usize),
    tool: Tool,
}

impl Node {
    fn min_minutes(&self, other: &Self) -> u32 {
        self.position.1.abs_diff(other.position.1) as u32
            + self.position.0.abs_diff(other.position.0) as u32
            + if other.tool != self.tool { 7 } else { 0 }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order by minutes - min heap
        (other.minutes + other.estimate)
            .cmp(&(self.minutes + self.estimate))
            .then_with(|| other.node.cmp(&self.node))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
enum Region {
    Rocky,
    Narrow,
    Wet,
}

impl Region {
    fn new(level: usize) -> Self {
        match level % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }

    fn tools(self) -> [Tool; 2] {
        match self {
            Region::Rocky => [Tool::ClimbingGear, Tool::Torch],
            Region::Wet => [Tool::ClimbingGear, Tool::Neither],
            Region::Narrow => [Tool::Torch, Tool::Neither],
        }
    }
}

impl FromStr for Cave {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(
            r"depth: (?P<depth>\d+)
target: (?P<tx>\d+),(?P<ty>\d+)",
        )?;
        let caps = regex.captures(input)?;
        let depth: usize = caps.name("depth")?.as_str().parse()?;
        let tx: usize = caps.name("tx")?.as_str().parse()?;
        let ty: usize = caps.name("ty")?.as_str().parse()?;
        //pad it so the search can go beyond the target down and to the right
        let cols = tx + 125;
        let rows = ty + 1;
        let regions = cart_product(0..rows, 0..cols)
            .scan(Vec::with_capacity(cols * rows), |levels, (y, x)| {
                let level = if x == 0 && y == 0 || x == tx && y == ty {
                    depth % 20183
                } else {
                    Cave::level(x, y, depth, || {
                        (levels[levels.len() - 1], levels[levels.len() - cols])
                    })
                };
                levels.push(level);
                Some(level)
            })
            .collect();

        Ok(Cave {
            regions,
            cols,
            depth,
            target: Node {
                position: (tx, ty),
                tool: Tool::Torch,
            },
        })
    }
}

#[derive(Debug)]
struct ParsingError;

impl From<regex::Error> for ParsingError {
    fn from(_: regex::Error) -> Self {
        ParsingError
    }
}

impl From<NoneError> for ParsingError {
    fn from(_: NoneError) -> Self {
        ParsingError
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.regions.iter().enumerate().try_for_each(|(i, a)| {
            let c = match Region::new(*a) {
                Region::Rocky => '.',
                Region::Narrow => '|',
                Region::Wet => '=',
            };

            if i % self.cols == self.cols - 1 {
                writeln!(f, "{}", c)
            } else {
                write!(f, "{}", c)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"depth: 510
target: 10,10
";
        assert_eq!(solve(input), 45);
    }
}

common::read_main!();
//common::bootstrap!(16);
