use std::collections::{hash_map, HashMap};

const NUM_CYCLES: usize = 1_000_000_000;

fn solve(input: &str) -> usize {
    let num_cols = input.trim().lines().map(|l| l.len()).next().unwrap();

    let mut platform: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                'O' => State::Circle,
                '#' => State::Block,
                '.' => State::Empty,
                _ => unreachable!(),
            })
        })
        .collect();

    let num_rows = platform.len() / num_cols;
    let mut memoized = HashMap::new();

    let get_load = |platform: &[State]| {
        platform
            .chunks(num_cols)
            .enumerate()
            .map(|(y, row)| {
                (num_rows - y) * row.iter().filter(|s| matches!(s, State::Circle)).count()
            })
            .sum()
    };

    let mut values: Vec<usize> = vec![];
    for i in 0..NUM_CYCLES {
        tilt_north(&mut platform, num_cols);
        tilt_west(&mut platform, num_rows);
        tilt_south(&mut platform, num_cols);
        tilt_east(&mut platform, num_rows);

        match memoized.entry(platform.clone()) {
            hash_map::Entry::Occupied(o) => {
                let offset = o.get();
                let cycle_len = i - offset;
                return values[offset + ((NUM_CYCLES - 1 - offset) % cycle_len)];
            }
            hash_map::Entry::Vacant(v) => {
                values.push(get_load(&platform));
                v.insert(i);
            }
        }
    }

    get_load(&platform)
}

fn tilt_north(platform: &mut [State], num_cols: usize) {
    // skip first row
    for i in num_cols..platform.len() {
        let State::Circle = platform[i] else { continue };
        let mut above = i - num_cols;
        while let State::Empty = platform[above] {
            let (first, second) = platform.split_at_mut(above + 1);
            std::mem::swap(&mut first[above], &mut second[num_cols - 1]);
            let Some(x) = above.checked_sub(num_cols) else {
                break;
            };
            above = x;
        }
    }
}

fn tilt_east(platform: &mut [State], num_rows: usize) {
    for i in (0..platform.len()).rev() {
        let State::Circle = platform[i] else { continue };
        let mut right = i + 1;

        while right % num_rows != 0 {
            let State::Empty = platform[right] else {
                break;
            };

            let (left_of, right_of) = platform.split_at_mut(right);
            std::mem::swap(&mut left_of[right - 1], &mut right_of[0]);

            right += 1;
        }
    }
}

fn tilt_south(platform: &mut [State], num_cols: usize) {
    // skip last row
    for i in (0..platform.len() - num_cols).rev() {
        let State::Circle = platform[i] else { continue };
        let mut below = i + num_cols;
        while let Some(State::Empty) = platform.get(below) {
            let (above_of, below_of) = platform.split_at_mut(below);
            std::mem::swap(&mut above_of[below - num_cols], &mut below_of[0]);

            below += num_cols;
        }
    }
}

fn tilt_west(platform: &mut [State], num_rows: usize) {
    for i in 0..platform.len() {
        let State::Circle = platform[i] else { continue };
        let Some(mut left) = i.checked_sub(1) else {
            continue;
        };

        while left % num_rows != num_rows - 1 {
            let State::Empty = platform[left] else {
                break;
            };

            let (left_of, right_of) = platform.split_at_mut(left + 1);
            std::mem::swap(&mut left_of[left], &mut right_of[0]);

            let Some(x) = left.checked_sub(1) else {
                break;
            };
            left = x;
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum State {
    Circle,
    Block,
    Empty,
}

common::read_main!();

#[test]
fn example() {
    let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    assert_eq!(solve(input), 64);
}
