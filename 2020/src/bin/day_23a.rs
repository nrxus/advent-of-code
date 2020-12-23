use std::collections::VecDeque;

fn solve(cups: &str) -> String {
    let mut ring: VecDeque<_> = cups
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    (0..100).for_each(|_| {
        let selected = *ring.front().unwrap();
        ring.rotate_left(1);

        let picked_up: Vec<_> = ring.drain(..3).collect();

        let destination = *ring
            .iter()
            .filter(|remaining| **remaining < selected)
            .max()
            .unwrap_or_else(|| ring.iter().max().unwrap());

        let index = ring
            .iter()
            .enumerate()
            .find_map(|(i, c)| if *c == destination { Some(i) } else { None })
            .unwrap();

        ring.rotate_left(index + 1);
        ring.extend(picked_up);
        ring.rotate_right(index + 4);
    });

    let length = ring.len();
    ring.into_iter()
        .cycle()
        .skip_while(|i| *i != 1)
        .skip(1)
        .take(length - 1)
        .map(|i| std::char::from_digit(i, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"389125467";
        assert_eq!(solve(input).to_string(), "67384529");
    }
}

common::read_main!();
