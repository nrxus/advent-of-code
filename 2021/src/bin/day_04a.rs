use std::{collections::HashMap, str::FromStr};

fn solve(input: &str) -> u32 {
    let input = input.trim();

    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let mut boards: Vec<_> = boards
        .split("\n\n")
        .map(Board::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    for number in numbers.split(',').map(|n| n.parse::<u32>().unwrap()) {
        if let Some(score) = boards.iter_mut().find_map(|b| b.mark(number)) {
            return score * number;
        }
    }

    panic!("no board won");
}

#[derive(Debug)]
struct Board {
    numbers: [Option<u32>; 25],
    // a copy of the above numbers for quick access
    positions: HashMap<u32, usize>,
}

impl Board {
    const LEN: usize = 5;

    pub fn mark(&mut self, number: u32) -> Option<u32> {
        let position = self.positions.remove(&number)?;
        self.numbers[position] = None;

        let column = position % Self::LEN;
        let finished_column = (0..Self::LEN).all(|row| {
            let position = row * Self::LEN + column;
            self.numbers[position].is_none()
        });

        if finished_column {
            return Some(self.positions.keys().sum());
        }

        let row = position / Self::LEN;
        let finished_row = (0..Self::LEN).all(|column| {
            let position = row * Self::LEN + column;
            self.numbers[position].is_none()
        });

        if finished_row {
            return Some(self.positions.keys().sum());
        }

        None
    }
}

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut board = Board {
            numbers: [None; 25],
            positions: HashMap::with_capacity(25),
        };

        input
            .split_whitespace()
            .enumerate()
            .try_for_each::<_, Result<(), Self::Err>>(|(i, n)| {
                let n = n.parse()?;
                board.numbers[i] = Some(n);
                let previous = board.positions.insert(n, i);

                // a valid board should never have the same number twice
                assert!(previous.is_none());

                Ok(())
            })?;

        // assert we filled it out
        assert!(board.positions.len() == Board::LEN * Board::LEN);

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(solve(input), 4512);
    }
}

common::read_main!();
