#![feature(try_trait)]

use common::extensions::cart_product;
use regex::Regex;
use std::{fmt, num::ParseIntError, option::NoneError, str::FromStr};

fn solve(input: &str) -> u32 {
    Cave::from_str(input).unwrap().danger_level()
}

impl Cave {
    fn danger_level(&self) -> u32 {
        self.regions
            .iter()
            .map(|r| match r {
                Region::Rocky => 0,
                Region::Wet => 1,
                Region::Narrow => 2,
            })
            .sum()
    }
}

struct Cave {
    regions: Vec<Region>,
    cols: usize,
}

enum Region {
    Rocky,
    Narrow,
    Wet,
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
        let cols = caps.name("tx")?.as_str().parse::<usize>()? + 1;
        let rows = caps.name("ty")?.as_str().parse::<usize>()? + 1;
        let regions = cart_product(0..rows, 0..cols)
            .scan(Vec::with_capacity(cols * rows), |levels, (y, x)| {
                let geo_index = if x == 0 && y == 0 || x == cols - 1 && y == rows - 1 {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    let col_before = levels[levels.len() - 1];
                    let row_before = levels[levels.len() - cols];
                    col_before * row_before
                };
                let level = (geo_index + depth) % 20183;
                levels.push(level);
                Some(level)
            })
            .map(|level| match level % 3 {
                0 => Region::Rocky,
                1 => Region::Wet,
                2 => Region::Narrow,
                _ => unreachable!(),
            })
            .collect();
        Ok(Cave { regions, cols })
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
            let c = match a {
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
        let cave = Cave::from_str(input).unwrap();
        let stringified = r".=.|=.|.|=.
.|=|=|||..|
.==|....||=
=.|....|.==
=|..==...=.
=||.=.=||=|
|.=.===|||.
|..==||=.|=
.=..===..=|
.======|||=
.===|=|===.";
        assert_eq!(cave.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 114);
    }
}

common::read_main!();
//common::bootstrap!(16);
