use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn solve(initial: &str) -> usize {
    let mut numbers: HashMap<usize, usize> = initial
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, number)| (number.parse().unwrap(), i + 1))
        .collect();

    let mut last = *numbers.iter().max_by_key(|(_, turn)| *turn).unwrap().0;
    numbers.remove(&last);

    for turn in numbers.len() + 2..=30000000 {
        match numbers.entry(last) {
            Entry::Vacant(v) => {
                last = 0;
                v.insert(turn - 1);
            }
            Entry::Occupied(mut o) => {
                last = turn - 1 - *o.get();
                o.insert(turn - 1);
            }
        }
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example() {
        let input = r"0,3,6";
        assert_eq!(solve(input), 175594);
    }

    #[test]
    #[ignore]
    fn example_two() {
        let input = r"1,3,2";
        assert_eq!(solve(input), 2578);
    }

    #[test]
    #[ignore]
    fn example_three() {
        let input = r"2,1,3";
        assert_eq!(solve(input), 3544142);
    }

    #[test]
    #[ignore]
    fn example_four() {
        let input = r"1,2,3";
        assert_eq!(solve(input), 261214);
    }

    #[test]
    #[ignore]
    fn example_five() {
        let input = r"2,3,1";
        assert_eq!(solve(input), 6895259);
    }

    #[test]
    #[ignore]
    fn example_six() {
        let input = r"3,2,1";
        assert_eq!(solve(input), 18);
    }

    #[test]
    #[ignore]
    fn example_seven() {
        let input = r"3,1,2";
        assert_eq!(solve(input), 362);
    }
}

common::read_main!();
