use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    input
        .trim()
        .as_bytes()
        .windows(4)
        .enumerate()
        .find_map(|(i, potential_marker)| {
            let set: HashSet<&u8> = HashSet::from_iter(potential_marker);
            if set.len() == 4 {
                Some(i + 4)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}

read_main!();
