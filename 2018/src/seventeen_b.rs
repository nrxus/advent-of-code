#![feature(try_trait)]

use std::{num::ParseIntError, ops::RangeInclusive, option::NoneError, str::FromStr};

use regex::Regex;

fn solve(input: &str) -> usize {
    let mut world: World = input.parse().unwrap();
    world.fill();
    world
        .tiles
        .into_iter()
        .filter(|t| match t {
            Tile::Water(WaterPath::Settled) => true,
            _ => false,
        })
        .count()
}

#[derive(Debug)]
struct World {
    tiles: Vec<Tile>,
    cols: usize,
    spring_x: usize,
}

impl World {
    fn fill(&mut self) {
        let mut queue = vec![self.spring_x];

        while let Some(tile) = queue.pop() {
            let next = self.drip(tile);
            queue.extend(next.iter().filter_map(|n| *n));
        }
    }

    fn drip(&mut self, index: usize) -> [Option<usize>; 3] {
        let mut water = match self.tiles[index] {
            Tile::Sand => WaterPath::Down,
            Tile::Water(w) => w,
            _ => unreachable!(),
        };

        let down = index + self.cols;
        if down >= self.tiles.len() {
            self.tiles[index] = Tile::Water(WaterPath::Down);
            return [None, None, None];
        }

        match self.tiles[down] {
            Tile::Clay | Tile::Water(WaterPath::Settled) => water.blocked_down(),
            _ => {}
        }

        let left = index - 1;
        match self.tiles[left] {
            Tile::Clay | Tile::Water(WaterPath::Settled) | Tile::Water(WaterPath::Right) => {
                water.blocked_left()
            }
            _ => {}
        }

        let right = index + 1;
        match self.tiles[right] {
            Tile::Clay | Tile::Water(WaterPath::Settled) | Tile::Water(WaterPath::Left) => {
                water.blocked_right()
            }
            _ => {}
        }

        self.tiles[index] = Tile::Water(water);
        match water {
            WaterPath::Down => match self.tiles[down] {
                Tile::Sand => [Some(index), Some(down), None],
                _ => [None, None, None],
            },
            WaterPath::Sides => match (self.tiles[left], self.tiles[right]) {
                (Tile::Sand, Tile::Sand) => [Some(index), Some(left), Some(right)],
                (Tile::Sand, _) => [Some(index), Some(left), None],
                (_, Tile::Sand) => [Some(index), None, Some(right)],
                _ => [None, None, None],
            },
            WaterPath::Left => match self.tiles[left] {
                Tile::Sand | Tile::Water(WaterPath::Sides) | Tile::Water(WaterPath::Right) => {
                    [Some(index), Some(left), None]
                }
                _ => [None, None, None],
            },
            WaterPath::Right => match self.tiles[right] {
                Tile::Sand | Tile::Water(WaterPath::Sides) | Tile::Water(WaterPath::Left) => {
                    [Some(index), Some(right), None]
                }
                _ => [None, None, None],
            },
            WaterPath::Settled => [None, None, None],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Sand,
    Water(WaterPath),
    Clay,
}

#[derive(Debug, Clone, Copy)]
enum WaterPath {
    Down,
    Sides,
    Left,
    Right,
    Settled,
}

impl WaterPath {
    fn blocked_down(&mut self) {
        if let WaterPath::Down = self {
            *self = WaterPath::Sides;
        }
    }

    fn blocked_right(&mut self) {
        match self {
            WaterPath::Sides => *self = WaterPath::Left,
            WaterPath::Right => *self = WaterPath::Settled,
            _ => {}
        }
    }

    fn blocked_left(&mut self) {
        match self {
            WaterPath::Sides => *self = WaterPath::Right,
            WaterPath::Left => *self = WaterPath::Settled,
            _ => {}
        }
    }
}

#[derive(Debug)]
enum ClayGroup {
    Horizontal(usize, RangeInclusive<usize>),
    Vertical(usize, RangeInclusive<usize>),
}

impl FromStr for World {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(?P<point_axis>[x|y])=(?P<point>\d+), (?P<range_axis>[x|y])=(?P<range_start>\d+)..(?P<range_end>\d+)")?;
        let groups: Vec<ClayGroup> = input
            .lines()
            .map(|l| {
                let caps = regex.captures(l)?;
                let point_axis = caps.name("point_axis")?.as_str();
                let point: usize = caps.name("point")?.as_str().parse()?;
                let range_axis = caps.name("range_axis")?.as_str();
                let range_start: usize = caps.name("range_start")?.as_str().parse()?;
                let range_end: usize = caps.name("range_end")?.as_str().parse()?;

                let range = range_start..=range_end;
                if point_axis == "x" && range_axis == "y" {
                    Ok(ClayGroup::Vertical(point, range))
                } else if point_axis == "y" && range_axis == "x" {
                    Ok(ClayGroup::Horizontal(point, range))
                } else {
                    Err(ParsingError)
                }
            })
            .collect::<Result<_, _>>()?;

        let min_x = groups
            .iter()
            .map(|g| match g {
                ClayGroup::Horizontal(_, range) => range.start(),
                ClayGroup::Vertical(x, _) => x,
            })
            .min()
            .map(|x| x - 1)?; //allow for spill on the left

        let max_x = groups
            .iter()
            .map(|g| match g {
                ClayGroup::Horizontal(_, range) => range.end(),
                ClayGroup::Vertical(x, _) => x,
            })
            .max()
            .map(|x| x + 1)?; //allow for spill on the top

        let min_y = groups
            .iter()
            .map(|g| match g {
                ClayGroup::Horizontal(y, _) => y,
                ClayGroup::Vertical(_, range) => range.start(),
            })
            .min()
            .cloned()?;

        let max_y = groups
            .iter()
            .map(|g| match g {
                ClayGroup::Horizontal(y, _) => y,
                ClayGroup::Vertical(_, range) => range.end(),
            })
            .max()
            .cloned()?;

        let cols = max_x - min_x + 1;
        let y_len = max_y - min_y + 1;

        let mut tiles = vec![Tile::Sand; cols * y_len];
        groups.into_iter().for_each(|g| match g {
            ClayGroup::Horizontal(y, range) => range.for_each(|x| {
                let x = x - min_x;
                let y = y - min_y;
                tiles[x + y * cols] = Tile::Clay;
            }),
            ClayGroup::Vertical(x, range) => range.for_each(|y| {
                let x = x - min_x;
                let y = y - min_y;
                tiles[x + y * cols] = Tile::Clay;
            }),
        });

        Ok(World {
            tiles,
            cols,
            spring_x: 500 - min_x,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        assert_eq!(solve(input), 29);
    }
}

common::read_main!();
//common::bootstrap!(17);

use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (0..self.cols)
            .map(|x| if x == self.spring_x { '+' } else { '.' })
            .try_for_each(|c| write!(f, "{}", c))?;
        writeln!(f)?;
        self.tiles.iter().enumerate().try_for_each(|(i, t)| {
            let c = match t {
                Tile::Clay => '#',
                Tile::Sand => '.',
                Tile::Water(WaterPath::Settled) => '~',
                Tile::Water(WaterPath::Down) => '|',
                Tile::Water(WaterPath::Sides) => '-',
                Tile::Water(WaterPath::Left) => '<',
                Tile::Water(WaterPath::Right) => '>',
            };

            if i % self.cols == self.cols - 1 {
                writeln!(f, "{}", c)
            } else {
                write!(f, "{}", c)
            }
        })
    }
}
