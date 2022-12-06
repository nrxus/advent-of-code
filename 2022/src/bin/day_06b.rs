use std::collections::HashSet;

use common::read_main;

fn solve(input: &str) -> usize {
    const LEN: usize = 14;

    input
        .trim()
        .as_bytes()
        .windows(LEN)
        .enumerate()
        .find_map(|(i, potential_marker)| {
            let set: HashSet<&u8> = HashSet::from_iter(potential_marker);
            if set.len() == LEN {
                Some(i + LEN)
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
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

read_main!();
