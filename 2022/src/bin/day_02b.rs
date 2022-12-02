use common::read_main;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|round| {
            let (theirs, outcome) = round.split_once(' ').unwrap();
            let theirs = match theirs {
                "A" => HandShape::Rock,
                "B" => HandShape::Paper,
                "C" => HandShape::Scissors,
                _ => unreachable!(),
            };
            let outcome = match outcome {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!(),
            };
            outcome.calculate_points(theirs)
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    pub fn calculate_points(self, against: HandShape) -> u32 {
        self as u32
            + match self {
                Outcome::Draw => against as u32,
                Outcome::Win => against as u32 % 3 + 1,
                Outcome::Lose => (against as u32 + 1) % 3 + 1,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"A Y
B X
C Z
";

        assert_eq!(solve(input), 12);
    }
}

read_main!();
