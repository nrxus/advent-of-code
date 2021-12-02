fn solve(input: &str) -> i32 {
    let position: Position = input
        .trim()
        .lines()
        .map(|command| command.parse().unwrap())
        .sum();

    position.x as i32 * position.y
}

#[derive(Default)]
struct Position {
    x: u32,
    y: i32,
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
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
            "down" => Command::Down(magnitude),
            "up" => Command::Up(magnitude),
            d => return Err(format!("bad direction: {}", d).into()),
        };

        Ok(command)
    }
}

impl std::ops::Add<Command> for Position {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Forward(magnitude) => Position {
                x: self.x + magnitude,
                y: self.y,
            },
            Command::Down(magnitude) => Position {
                x: self.x,
                y: self.y + magnitude as i32,
            },
            Command::Up(magnitude) => Position {
                x: self.x,
                y: self.y - (magnitude as i32),
            },
        }
    }
}

impl std::iter::Sum<Command> for Position {
    fn sum<I: Iterator<Item = Command>>(iter: I) -> Self {
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
        assert_eq!(solve(input), 150);
    }
}

common::read_main!();
