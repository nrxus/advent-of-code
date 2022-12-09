use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    let mut rope = [(0, 0); 10];
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert(rope[9]);
    input.trim().lines().for_each(|motion| {
        let (direction, distance) = motion.split_once(' ').unwrap();
        let distance: u8 = distance.parse().unwrap();
        match direction {
            "R" => (0..distance).for_each(|_| {
                rope[0].0 += 1;
                catch_up(&mut rope);
                visited.insert(rope[9]);
            }),
            "L" => (0..distance).for_each(|_| {
                rope[0].0 -= 1;
                catch_up(&mut rope);
                visited.insert(rope[9]);
            }),
            "U" => (0..distance).for_each(|_| {
                rope[0].1 += 1;
                catch_up(&mut rope);
                visited.insert(rope[9]);
            }),
            "D" => (0..distance).for_each(|_| {
                rope[0].1 -= 1;
                catch_up(&mut rope);
                visited.insert(rope[9]);
            }),
            d => panic!("unhandled direction: {d}"),
        }
    });

    visited.len()
}

fn catch_up(rope: &mut [(i16, i16); 10]) {
    for i in 1..rope.len() {
        let (head, tail) = rope.split_at_mut(i);
        let head = head.last().unwrap();
        let tail = &mut tail[0];
        if !follow(tail, head) {
            break;
        }
    }
}

fn follow(tail: &mut (i16, i16), head: &(i16, i16)) -> bool {
    if *tail == *head {
        return false;
    }

    if tail.0 == head.0 {
        let diff = head.1 - tail.1;
        if diff.abs() == 1 {
            return false;
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
            return false;
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
            return false;
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

    true
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
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn larger_example() {
        let input = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(solve(input), 36);
    }
}

read_main!();
