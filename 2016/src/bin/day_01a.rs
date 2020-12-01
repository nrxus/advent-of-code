fn solve(directions: &str) -> u32 {
    directions
        .split(", ")
        .map(|d| d.parse().expect("failed to parse"))
        .sum::<Position>()
        .distance()
}

#[derive(Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

enum Step {
    Left(u32),
    Right(u32),
}

impl std::iter::Sum<Step> for Position {
    fn sum<I: Iterator<Item = Step>>(iter: I) -> Self {
        iter.fold(Position::default(), std::ops::Add::add)
    }
}

impl std::ops::Add<Step> for Position {
    type Output = Position;

    fn add(self, rhs: Step) -> Self {
        match (self.direction, rhs) {
            (Direction::North, Step::Left(step)) | (Direction::South, Step::Right(step)) => {
                Position {
                    direction: Direction::West,
                    x: self.x + step as i32,
                    ..self
                }
            }
            (Direction::North, Step::Right(step)) | (Direction::South, Step::Left(step)) => {
                Position {
                    direction: Direction::East,
                    x: self.x - step as i32,
                    ..self
                }
            }
            (Direction::East, Step::Left(step)) | (Direction::West, Step::Right(step)) => {
                Position {
                    direction: Direction::North,
                    y: self.y + step as i32,
                    ..self
                }
            }
            (Direction::East, Step::Right(step)) | (Direction::West, Step::Left(step)) => {
                Position {
                    direction: Direction::South,
                    y: self.y - step as i32,
                    ..self
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
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
struct ParsingError;

impl std::str::FromStr for Step {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = &s[0..1];
        let n_blocks: u32 = s[1..].parse().map_err(|_| ParsingError)?;
        match turn {
            "L" => Ok(Step::Left(n_blocks)),
            "R" => Ok(Step::Right(n_blocks)),
            _ => Err(ParsingError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(solve("R2, L3"), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(solve("R2, R2, R2"), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(solve("R5, L5, R5, R3"), 12);
    }
}

common::read_main!();
