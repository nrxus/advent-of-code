use std::fmt;

fn solve(instructions: &str) -> BathroomCode {
    let digits = instructions
        .trim()
        .lines()
        .map(|i| {
            i.chars().map(|c| match c {
                'U' => Direction::Up,
                'L' => Direction::Left,
                'R' => Direction::Right,
                'D' => Direction::Down,
                _ => panic!("unexpected direction"),
            })
        })
        .scan(FingerPosition::default(), |position, instructions| {
            instructions.for_each(|direction| position.choose(direction));
            Some(position.digit())
        })
        .collect();

    BathroomCode { digits }
}

struct BathroomCode {
    digits: Vec<Digit>,
}

#[derive(Clone, Copy)]
enum Digit {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

struct FingerPosition(usize, usize);

const DIGIT_PAD: [[Digit; 3]; 3] = [
    [Digit::One, Digit::Two, Digit::Three],
    [Digit::Four, Digit::Five, Digit::Six],
    [Digit::Seven, Digit::Eight, Digit::Nine],
];

impl Default for FingerPosition {
    fn default() -> Self {
        FingerPosition(1, 1)
    }
}

impl FingerPosition {
    fn choose(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.0 = self.0.saturating_sub(1),
            Direction::Up => self.1 = self.1.saturating_sub(1),
            Direction::Right => self.0 = (self.0 + 1).min(DIGIT_PAD.len() - 1),
            Direction::Down => self.1 = (self.1 + 1).min(DIGIT_PAD.len() - 1),
        };
    }

    fn digit(&self) -> Digit {
        DIGIT_PAD[self.1][self.0]
    }
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl fmt::Display for BathroomCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.digits
            .iter()
            .try_for_each(|d| write!(f, "{}", *d as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = r"ULL
RRDDD
LURDL
UUUUD";
        assert_eq!(format!("{}", solve(instructions)), "1985");
    }
}

common::read_main!();
