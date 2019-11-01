#![feature(try_trait)]

use std::{num::ParseIntError, option::NoneError};

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

    let max_nano = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    nanobots
        .iter()
        .filter(|n| max_nano.can_reach(n.pos))
        .count()
}

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
        let input = r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        assert_eq!(solve(input), 7);
    }
}

common::read_main!();
//common::bootstrap!(16);
