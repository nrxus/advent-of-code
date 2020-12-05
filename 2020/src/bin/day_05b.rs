use bitvec::prelude::{bitarr, Lsb0};
use std::collections::HashSet;

fn solve(seats: &str) -> u16 {
    let all_seats: HashSet<u16> = (0..1024).collect();
    let full_seats: HashSet<_> = seats.trim().lines().map(seat_id).collect();
    let empty_seats: HashSet<_> = all_seats.difference(&full_seats).cloned().collect();
    let potential_seats: Vec<_> = empty_seats
        .into_iter()
        .filter(|&s| full_seats.contains(&(s + 1)) && full_seats.contains(&(s - 1)))
        .collect();

    if potential_seats.len() != 1 {
        panic!(
            "bug: potential seats is not a single seat {:?}",
            potential_seats
        );
    }

    potential_seats[0]
}

fn seat_id(seat: &str) -> u16 {
    let row = seat[0..7]
        .chars()
        .rev()
        .enumerate()
        .fold(bitarr![Lsb0, u8; 0; 8], |mut column, (i, c)| {
            match c {
                'F' => {}
                'B' => column.set(i, true),
                c => panic!("unexpected: {:?}", c),
            };
            column
        })
        .as_slice()[0];

    let column = seat[7..10]
        .chars()
        .rev()
        .enumerate()
        .fold(bitarr![Lsb0, u8; 0; 3], |mut column, (i, c)| {
            match c {
                'L' => {}
                'R' => column.set(i, true),
                c => panic!("unexpected: {:?}", c),
            };
            column
        })
        .as_slice()[0];

    (row as u16) * 8 + (column as u16)
}

common::read_main!();
