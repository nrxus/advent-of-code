fn solve(input: &str) -> i32 {
    let position: Position = input
        .trim()
        .lines()
        .map(|command| command.parse().unwrap())
        .scan(0, |aim, command| {
            let delta = match command {
                Command::Aim(aim_delta) => {
                    *aim += aim_delta;
                    Vector::default()
                }
                Command::Forward(magnitude) => Vector {
                    x: magnitude,
                    y: *aim * magnitude as i32,
                },
            };

            Some(delta)
        })
        .sum();

    position.x as i32 * position.y
}

enum Command {
    Aim(i32),
    Forward(u32),
}

impl std::str::FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = command
            .split_once(' ')
            .ok_or_else(|| String::from("invalid command line"))?;

        let magnitude: u32 = magnitude.parse()?;

        let command = match direction {
            "forward" => Command::Forward(magnitude),
            "down" => Command::Aim(magnitude as i32),
            "up" => Command::Aim(-(magnitude as i32)),
            d => return Err(format!("bad direction: {}", d).into()),
        };

        Ok(command)
    }
}

#[derive(Default)]
struct Position {
    x: u32,
    y: i32,
}

#[derive(Default)]
struct Vector {
    x: u32,
    y: i32,
}

impl std::ops::Add<Vector> for Position {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::iter::Sum<Vector> for Position {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Self {
        iter.fold(Position::default(), |position, delta| position + delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(solve(input), 900);
    }
}

common::read_main!();
