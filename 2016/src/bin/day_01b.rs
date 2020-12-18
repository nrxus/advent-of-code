use std::collections::HashSet;

fn solve(directions: &str) -> u32 {
    let mut position = Position::default();
    let mut previous = HashSet::new();

    'outer: for step in directions
        .trim()
        .split(", ")
        .map(|d| d.parse::<Step>().expect("failed to parse"))
    {
        let next = position + step;
        match next.direction {
            Direction::North => {
                for y in position.y + 1..=next.y {
                    if !previous.insert((next.x, y)) {
                        position.y = y;
                        break 'outer;
                    }
                }
            }
            Direction::South => {
                for y in (next.y..position.y).rev() {
                    if !previous.insert((next.x, y)) {
                        position.y = y;
                        break 'outer;
                    }
                }
            }
            Direction::East => {
                for x in position.x + 1..=next.x {
                    if !previous.insert((x, next.y)) {
                        position.x = x;
                        break 'outer;
                    }
                }
            }
            Direction::West => {
                for x in (next.x..position.x).rev() {
                    if !previous.insert((x, next.y)) {
                        position.x = x;
                        break 'outer;
                    }
                }
            }
        }
        position = next;
    }

    position.distance()
}

#[derive(Default, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

struct Step {
    blocks: u32,
    turn: Turn,
}

impl std::ops::Add<Step> for Position {
    type Output = Position;

    fn add(self, rhs: Step) -> Self {
        // first turn
        let direction = self.direction + rhs.turn;

        // then advance blocks
        let blocks = rhs.blocks as i32;
        let (x, y) = match direction {
            Direction::North => (self.x, self.y + blocks),
            Direction::South => (self.x, self.y - blocks),
            Direction::East => (self.x + blocks, self.y),
            Direction::West => (self.x - blocks, self.y),
        };

        Position { x, y, direction }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Turn {
    Left,
    Right,
}

impl std::ops::Add<Turn> for Direction {
    type Output = Direction;

    fn add(self, rhs: Turn) -> Self {
        match (self, rhs) {
            (Direction::North, Turn::Left) | (Direction::South, Turn::Right) => Direction::West,
            (Direction::North, Turn::Right) | (Direction::South, Turn::Left) => Direction::East,
            (Direction::East, Turn::Left) | (Direction::West, Turn::Right) => Direction::North,
            (Direction::East, Turn::Right) | (Direction::West, Turn::Left) => Direction::South,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl Position {
    fn distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

#[derive(Debug)]
enum ParsingError {
    Turn,
    Block,
}

impl std::str::FromStr for Step {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = match &s[0..1] {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(ParsingError::Turn),
        }?;

        let blocks: u32 = s[1..].parse().map_err(|_| ParsingError::Block)?;

        Ok(Step { turn, blocks })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("R8, R4, R4, R8"), 4);
    }
}

common::read_main!();
