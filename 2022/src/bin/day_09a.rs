use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = head;
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert(tail);
    input.trim().lines().for_each(|motion| {
        let (direction, distance) = motion.split_once(' ').unwrap();
        let distance: u8 = distance.parse().unwrap();
        match direction {
            "R" => (0..distance).for_each(|_| {
                head.0 += 1;
                catch_up(&mut tail, &head);
                visited.insert(tail);
            }),
            "L" => (0..distance).for_each(|_| {
                head.0 -= 1;
                catch_up(&mut tail, &head);
                visited.insert(tail);
            }),
            "U" => (0..distance).for_each(|_| {
                head.1 += 1;
                catch_up(&mut tail, &head);
                visited.insert(tail);
            }),
            "D" => (0..distance).for_each(|_| {
                head.1 -= 1;
                catch_up(&mut tail, &head);
                visited.insert(tail);
            }),
            d => panic!("unhandled direction: {d}"),
        }
    });

    visited.len()
}

fn catch_up(tail: &mut (i16, i16), head: &(i16, i16)) {
    if *tail == *head {
        return;
    }

    if tail.0 == head.0 {
        let diff = head.1 - tail.1;
        if diff.abs() == 1 {
            return;
        }
        if diff == 2 {
            tail.1 += 1;
        } else if diff == -2 {
            tail.1 -= 1;
        } else {
            unreachable!()
        }
    } else if tail.1 == head.1 {
        let diff = head.0 - tail.0;
        if diff.abs() == 1 {
            return;
        }
        if diff == 2 {
            tail.0 += 1;
        } else if diff == -2 {
            tail.0 -= 1;
        } else {
            unreachable!()
        }
    } else {
        let x_diff = head.0 - tail.0;
        let y_diff = head.1 - tail.1;
        if x_diff.abs() == 1 && y_diff.abs() == 1 {
            return;
        }
        if x_diff.is_positive() {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
        if y_diff.is_positive() {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(solve(input), 13);
    }
}

read_main!();
