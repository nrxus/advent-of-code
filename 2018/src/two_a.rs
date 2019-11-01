use enum_map::{Enum, EnumMap};

use std::collections::HashSet;

#[derive(Debug, Enum)]
enum Count {
    One,
    Two,
    Three,
    TooMany,
}

fn solve(input: &str) -> i32 {
    let (twos, threes) = input
        .lines()
        .map(|id| {
            id.chars()
                .fold(EnumMap::<_, HashSet<_>>::new(), |mut count, c| {
                    if count[Count::TooMany].contains(&c) {
                        //do nothing
                    } else if count[Count::Three].contains(&c) {
                        count[Count::Three].remove(&c);
                        count[Count::TooMany].insert(c);
                    } else if count[Count::Two].contains(&c) {
                        count[Count::Two].remove(&c);
                        count[Count::Three].insert(c);
                    } else if count[Count::One].contains(&c) {
                        count[Count::One].remove(&c);
                        count[Count::Two].insert(c);
                    } else {
                        count[Count::One].insert(c);
                    }
                    count
                })
        })
        .map(|counts| {
            (
                !counts[Count::Two].is_empty(),
                !counts[Count::Three].is_empty(),
            )
        })
        .fold((0, 0), |sum, (matches_twice, matches_thrice)| {
            (sum.0 + matches_twice as u32, sum.1 + matches_thrice as u32)
        });
    (twos * threes) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#
            ),
            12
        );
    }
}

common::bootstrap!(2);
