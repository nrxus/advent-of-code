use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {
    static ref DIGIT_PAD: HashMap<(usize, usize), char> = vec![
        ((2, 0), '1'),
        ((1, 1), '2'),
        ((2, 1), '3'),
        ((3, 1), '4'),
        ((0, 2), '5'),
        ((1, 2), '6'),
        ((2, 2), '7'),
        ((3, 2), '8'),
        ((4, 2), '9'),
        ((1, 3), 'A'),
        ((2, 3), 'B'),
        ((3, 3), 'C'),
        ((2, 4), 'D'),
    ]
    .into_iter()
    .collect();
}

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
    digits: Vec<char>,
}

#[derive(Clone, Copy)]
struct FingerPosition(usize, usize);

impl Default for FingerPosition {
    fn default() -> Self {
        FingerPosition(0, 2)
    }
}

impl FingerPosition {
    fn choose(&mut self, direction: Direction) {
        let mut pos = *self;

        match direction {
            Direction::Left => pos.0 = self.0.saturating_sub(1),
            Direction::Up => pos.1 = self.1.saturating_sub(1),
            Direction::Right => pos.0 = self.0 + 1,
            Direction::Down => pos.1 = self.1 + 1,
        };

        if DIGIT_PAD.contains_key(&(pos.0, pos.1)) {
            *self = pos
        }
    }

    fn digit(&self) -> char {
        DIGIT_PAD[&(self.0, self.1)]
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
        self.digits.iter().try_for_each(|c| write!(f, "{}", c))
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
        assert_eq!(format!("{}", solve(instructions)), "5DB3");
    }
}

common::read_main!();
