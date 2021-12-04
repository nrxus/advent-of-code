use std::{collections::HashMap, str::FromStr};

fn solve(input: &str) -> u32 {
    let input = input.trim();

    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let mut boards: Vec<_> = boards
        .split("\n\n")
        .map(Board::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    let mut last_score = None;

    for number in numbers.split(',').map(|n| n.parse::<u32>().unwrap()) {
        // someday we will have retain_mut or drain_filter so we can
        // have nice things. Until then keep track of all the boards
        // we are going to remove later
        let mut boards_to_remove = vec![];

        for (i, board) in &mut boards.iter_mut().enumerate() {
            if let Some(score) = board.mark(number) {
                last_score = Some(score);
                boards_to_remove.push(i);
            }
        }

        boards_to_remove
            .into_iter()
            .enumerate()
            .for_each(|(i, board_i)| {
                // as we remove boards they get shifted back so we
                // need to take that into account
                boards.remove(board_i - i);
            });
    }

    last_score.expect("no boards won")
}

/// 5x5 Bingo Board
#[derive(Debug)]
pub struct Board {
    numbers: [Option<u32>; Self::LEN * Self::LEN],
    // a copy of the above numbers for quick access
    positions: HashMap<u32, usize>,
}

impl Board {
    const LEN: usize = 5;

    /// Marks a number as seen in the board
    /// returns the score if the board was won
    pub fn mark(&mut self, number: u32) -> Option<u32> {
        let position = self.positions.remove(&number)?;
        self.numbers[position] = None;

        if self.finished(position) {
            Some(self.sum_remaining() * number)
        } else {
            None
        }
    }

    fn finished(&self, position_hint: usize) -> bool {
        let column_hint = position_hint % Self::LEN;
        let row_hint = position_hint / Self::LEN;

        (0..Self::LEN).all(|row| self.is_marked(row, column_hint))
            || (0..Self::LEN).all(|column| self.is_marked(row_hint, column))
    }

    fn sum_remaining(&self) -> u32 {
        self.positions.keys().sum()
    }

    fn is_marked(&self, row: usize, column: usize) -> bool {
        let position = row * Self::LEN + column;
        self.numbers[position].is_none()
    }
}

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        const SIZE: usize = Board::LEN * Board::LEN;

        let mut board = Board {
            numbers: [None; SIZE],
            positions: HashMap::with_capacity(SIZE),
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
        assert!(board.positions.len() == SIZE);

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
        assert_eq!(solve(input), 1924);
    }
}

common::read_main!();
