use common::read_main;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|round| {
            let (theirs, mine) = round.split_once(' ').unwrap();
            let theirs = match theirs {
                "A" => HandShape::Rock,
                "B" => HandShape::Paper,
                "C" => HandShape::Scissors,
                _ => unreachable!(),
            };
            let mine = match mine {
                "X" => HandShape::Rock,
                "Y" => HandShape::Paper,
                "Z" => HandShape::Scissors,
                _ => unreachable!(),
            };
            mine.against(theirs)
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl HandShape {
    pub fn against(self, other: Self) -> u32 {
        self as u32
            + match (self, other) {
                // LOST
                (HandShape::Rock, HandShape::Paper)
                | (HandShape::Paper, HandShape::Scissors)
                | (HandShape::Scissors, HandShape::Rock) => 0,
                // TIED
                (HandShape::Rock, HandShape::Rock)
                | (HandShape::Paper, HandShape::Paper)
                | (HandShape::Scissors, HandShape::Scissors) => 3,
                // WON
                (HandShape::Rock, HandShape::Scissors)
                | (HandShape::Paper, HandShape::Rock)
                | (HandShape::Scissors, HandShape::Paper) => 6,
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

        assert_eq!(solve(input), 15);
    }
}

read_main!();
