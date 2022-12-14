use std::{collections::HashSet, ops::RangeInclusive};

use common::read_main;

fn solve(input: &str) -> usize {
    let mut map = Map {
        occupied: HashSet::new(),
        // probably not needed but it's technically more correct
        floor_depth: 2,
    };

    input
        .trim()
        .lines()
        .map(|path| {
            path.split(" -> ").map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
            })
        })
        .for_each(|mut path| {
            let mut start = path.next().unwrap();
            path.for_each(|end| {
                if start.0 == end.0 {
                    map.add_wall(start.0, start.1.min(end.1)..=start.1.max(end.1));
                } else if start.1 == end.1 {
                    map.add_floor(start.1, start.0.min(end.0)..=start.0.max(end.0));
                } else {
                    panic!("not a straight line: {start:?} -> {end:?}")
                }
                start = end;
            });
        });

    let mut num_sand_dropped = 0;
    while map.add_sand() {
        num_sand_dropped += 1;
    }
    num_sand_dropped
}

#[derive(Debug, Default)]
struct Map {
    occupied: HashSet<(u16, u16)>,
    floor_depth: u16,
}

impl Map {
    pub fn add_floor(&mut self, depth: u16, floor: impl Iterator<Item = u16>) {
        self.occupied.extend(floor.map(|x| (x, depth)));
        self.floor_depth = self.floor_depth.max(depth + 2);
    }

    pub fn add_wall(&mut self, x: u16, wall: RangeInclusive<u16>) {
        self.floor_depth = self.floor_depth.max(wall.end() + 2);
        self.occupied.extend(wall.map(|y| (x, y)));
    }

    pub fn add_sand(&mut self) -> bool {
        let mut pos = (500, 0);
        if self.occupied.contains(&pos) {
            return false;
        }

        loop {
            // down
            let mut next = (pos.0, pos.1 + 1);

            // reached the floor
            if next.1 == self.floor_depth {
                self.occupied.insert(pos);
                break true;
            }

            if !self.occupied.contains(&next) {
                pos = next;
                continue;
            }

            // down left
            next.0 -= 1;
            if !self.occupied.contains(&next) {
                pos = next;
                continue;
            }

            // down right
            next.0 += 2;
            if !self.occupied.contains(&next) {
                pos = next;
                continue;
            }

            // settled
            self.occupied.insert(pos);
            break true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
        assert_eq!(solve(input), 93);
    }
}

read_main!();
