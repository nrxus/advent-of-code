use std::collections::HashMap;

fn solve(input: &str) -> u64 {
    let mut ratings: Vec<u32> = std::iter::once(0)
        .chain(input.trim().lines().map(|l| l.parse().unwrap()))
        .collect();

    ratings.sort_unstable();

    let mut counts = HashMap::new();
    counts.insert(0, 1);

    for (i, goal) in ratings[..ratings.len() - 1].iter().enumerate() {
        let count = counts.remove(&goal).unwrap();

        ratings[i + 1..]
            .iter()
            .take_while(|r| **r <= goal + 3)
            .for_each(|r| *counts.entry(*r).or_insert(0) += count);
    }

    if counts.len() != 1 {
        panic!(
            "bug: final counts should be exactly one option {:?}",
            counts
        )
    }

    counts.into_iter().map(|(_, v)| v).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn example_two() {
        let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(solve(input), 19208);
    }

    #[test]
    fn example_three() {
        let input = r"1
4
5
7
8
";
        assert_eq!(solve(input), 3);
    }
}

common::read_main!();
