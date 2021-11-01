use bitvec::prelude::{bitarr, BitStore, Lsb0};

fn solve(seats: &str) -> u32 {
    seats.trim().lines().map(cost_id).max().unwrap()
}

fn cost_id(seat: &str) -> u32 {
    let row = seat[0..7]
        .chars()
        .rev()
        .enumerate()
        .fold(bitarr![Lsb0, u32; 0; 8], |mut column, (i, c)| {
            match c {
                'F' => {}
                'B' => column.set(i, true),
                c => panic!("unexpected: {:?}", c),
            };
            column
        })
        .as_raw_slice()[0];

    let column = seat[7..10]
        .chars()
        .rev()
        .enumerate()
        .fold(bitarr![Lsb0, u32; 0; 3], |mut column, (i, c)| {
            match c {
                'L' => {}
                'R' => column.set(i, true),
                c => panic!("unexpected: {:?}", c),
            };
            column
        })
        .as_raw_slice()[0];

    row * 8 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"FBFBBFFRLR";
        assert_eq!(solve(input), 357);
    }

    #[test]
    fn example_two() {
        let input = r"BFFFBBFRRR";
        assert_eq!(solve(input), 567);
    }

    #[test]
    fn example_three() {
        let input = r"FFFBBBFRRR";
        assert_eq!(solve(input), 119);
    }

    #[test]
    fn example_four() {
        let input = r"BBFFBBFRLL";
        assert_eq!(solve(input), 820);
    }
}

common::read_main!();
