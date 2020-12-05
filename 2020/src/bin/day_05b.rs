use std::collections::HashSet;

fn solve(seats: &str) -> u16 {
    let all_seats: HashSet<u16> = (0..1024).collect();
    let occupied_seats: HashSet<_> = seats.trim().lines().map(seat_id).collect();
    let empty_seats: HashSet<_> = all_seats.difference(&occupied_seats).cloned().collect();
    let potential_seats: Vec<_> = empty_seats
        .into_iter()
        .filter(|&s| occupied_seats.contains(&(s + 1)) && occupied_seats.contains(&(s - 1)))
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
    let row: String = seat[0..7]
        .chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            c => panic!("unexpected: {:?}", c),
        })
        .collect();

    let row = u16::from_str_radix(&row, 2).unwrap();

    let column: String = seat[0..7]
        .chars()
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            c => panic!("unexpected: {:?}", c),
        })
        .collect();

    let column = u16::from_str_radix(&column, 2).unwrap();

    row * 8 + (column as u16)
}

common::read_main!();
