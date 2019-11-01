#![feature(try_trait, drain_filter)]

use std::{cmp::Ordering, collections::BinaryHeap, num::ParseIntError, option::NoneError};

use array_macro::array;
use regex::Regex;

fn solve(input: &str) -> usize {
    let regex = Regex::new(r"pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, r=(?P<r>\d+)").unwrap();
    let nanobots: Vec<_> = input
        .trim()
        .lines()
        .map(|l| {
            let caps = regex.captures(l)?;
            let x = caps.name("x")?.as_str().parse()?;
            let y = caps.name("y")?.as_str().parse()?;
            let z = caps.name("z")?.as_str().parse()?;
            let radius = caps.name("r")?.as_str().parse()?;
            let pos = (x, y, z);
            Ok(Nanobot { pos, radius })
        })
        .collect::<Result<_, ParsingError>>()
        .unwrap();

    let x_min = nanobots.iter().map(|n| n.pos.0).min().unwrap();
    let x_max = nanobots.iter().map(|n| n.pos.0).max().unwrap();
    let y_min = nanobots.iter().map(|n| n.pos.1).min().unwrap();
    let y_max = nanobots.iter().map(|n| n.pos.1).max().unwrap();
    let z_min = nanobots.iter().map(|n| n.pos.2).min().unwrap();
    let z_max = nanobots.iter().map(|n| n.pos.2).max().unwrap();
    let region = Region {
        nanobots: nanobots.iter().collect(),
        bounds: Bounds {
            pos: (x_min, y_min, z_min),
            length: (
                (x_max - x_min + 1) as usize,
                (y_max - y_min + 1) as usize,
                (z_max - z_min + 1) as usize,
            ),
        },
    };

    let mut queue = BinaryHeap::new();
    queue.push(region);
    let region = loop {
        let r = queue.pop().unwrap();
        if r.bounds.length == (1, 1, 1) {
            break r;
        }
        queue.extend(
            r.divide()
                .iter()
                .filter(|r| r.bounds.length.0 > 0 && r.bounds.length.1 > 0 && r.bounds.length.2 > 0)
                .filter(|r| r.nanobots.len() > 1)
                .cloned(),
        );
    };

    let pos = region.bounds.pos;
    (pos.0.abs() + pos.1.abs() + pos.2.abs()) as usize
}

#[derive(Debug, PartialEq, Eq)]
struct Nanobot {
    pos: (isize, isize, isize),
    radius: usize,
}

impl Nanobot {
    fn can_reach(&self, pos: (isize, isize, isize)) -> bool {
        ((self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs() + (self.pos.2 - pos.2).abs())
            as usize
            <= self.radius
    }

    fn intersects(&self, bounds: &Bounds) -> bool {
        bounds.corners().iter().any(|&c| self.can_reach(c))
            || self.can_reach(bounds.center())
            || self.edges().iter().any(|&e| bounds.surrounds(e))
    }

    fn edges(&self) -> [(isize, isize, isize); 6] {
        let (x, y, z) = self.pos;
        [
            (x, y, z - self.radius as isize),
            (x, y, z + self.radius as isize),
            (x, y - self.radius as isize, z),
            (x, y + self.radius as isize, z),
            (x - self.radius as isize, y, z),
            (x - self.radius as isize, y, z),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bounds {
    pos: (isize, isize, isize),
    length: (usize, usize, usize),
}

impl Bounds {
    fn divide(&self) -> [Bounds; 8] {
        let (left, right) = Bounds::ldiv(self.length.0);
        let (up, down) = Bounds::ldiv(self.length.1);
        let (inwards, outwards) = Bounds::ldiv(self.length.2);
        let (x, y, z) = self.pos;

        [
            Bounds {
                pos: (x, y, z),
                length: (left, up, inwards),
            },
            Bounds {
                pos: (x, y, z + inwards as isize),
                length: (left, up, outwards),
            },
            Bounds {
                pos: (x, y + up as isize, z),
                length: (left, down, inwards),
            },
            Bounds {
                pos: (x, y + up as isize, z + inwards as isize),
                length: (left, down, outwards),
            },
            Bounds {
                pos: (x + left as isize, y, z),
                length: (right, up, inwards),
            },
            Bounds {
                pos: (x + left as isize, y, z + inwards as isize),
                length: (right, up, outwards),
            },
            Bounds {
                pos: (x + left as isize, y + up as isize, z),
                length: (right, down, inwards),
            },
            Bounds {
                pos: (x + left as isize, y + up as isize, z + inwards as isize),
                length: (right, down, outwards),
            },
        ]
    }

    fn corners(&self) -> [(isize, isize, isize); 8] {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.length;
        let (dx, dy, dz) = (dx as isize, dy as isize, dz as isize);

        [
            (x, y, z),
            (x, y, z + dz - 1),
            (x, y + dy - 1, z),
            (x, y + dy - 1, z + dz - 1),
            (x + dx - 1, y, z),
            (x + dx - 1, y, z + dz - 1),
            (x + dx - 1, y + dy - 1, z),
            (x + dx - 1, y + dy - 1, z + dz - 1),
        ]
    }

    fn center(&self) -> (isize, isize, isize) {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.length;
        (
            x + dx as isize / 2,
            y + dy as isize / 2,
            z + dz as isize / 2,
        )
    }

    fn surrounds(&self, (x, y, z): (isize, isize, isize)) -> bool {
        let (left, top, inwards) = self.pos;
        let (right, bottom, outwards) = (
            left + self.length.0 as isize,
            top + self.length.1 as isize,
            inwards + self.length.2 as isize,
        );
        x >= left && x < right && y >= top && y < bottom && z >= inwards && z < outwards
    }

    fn area(&self) -> usize {
        self.length.0 * self.length.1 * self.length.2
    }

    fn dist_to_origin(&self) -> usize {
        let (left, top, inwards) = self.pos;
        let (right, bottom, outwards) = (
            left + self.length.0 as isize,
            top + self.length.1 as isize,
            inwards + self.length.2 as isize,
        );

        let x_dist = if right < 0 {
            (-right) as usize
        } else if left > 0 {
            left as usize
        } else {
            0
        };
        let y_dist = if bottom < 0 {
            (-bottom) as usize
        } else if top > 0 {
            top as usize
        } else {
            0
        };
        let z_dist = if outwards < 0 {
            (-outwards) as usize
        } else if inwards > 0 {
            inwards as usize
        } else {
            0
        };
        x_dist + y_dist + z_dist
    }

    fn ldiv(len: usize) -> (usize, usize) {
        if len % 2 == 0 {
            (len / 2, len / 2)
        } else {
            (len / 2, len / 2 + 1)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region<'n> {
    bounds: Bounds,
    nanobots: Vec<&'n Nanobot>,
}

impl Ord for Region<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let len_cmp = self.nanobots.len().cmp(&other.nanobots.len());
        if len_cmp == Ordering::Equal {
            let to_origin = self.bounds.dist_to_origin();
            let other_to_origin = self.bounds.dist_to_origin();
            let dist_cmp = other_to_origin.cmp(&to_origin);
            if dist_cmp == Ordering::Equal {
                other.bounds.area().cmp(&self.bounds.area())
            } else {
                dist_cmp
            }
        } else {
            len_cmp
        }
    }
}

impl PartialOrd for Region<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'n> Region<'n> {
    fn divide(&self) -> [Region<'n>; 8] {
        let boxes = self.bounds.divide();
        array![|i| Region {
            bounds: boxes[i],
            nanobots: self.nanobots.iter().filter(|n| n.intersects(&boxes[i])).cloned().collect()
        }; 8]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        assert_eq!(solve(input), 36);
    }
}

common::read_main!();
//common::bootstrap!(16);
