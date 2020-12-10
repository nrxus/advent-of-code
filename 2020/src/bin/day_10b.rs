use std::collections::HashMap;

fn solve(input: &str) -> u64 {
    let mut ratings: Vec<u32> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    ratings.sort_unstable();

    let mut last = 0;
    let mut counts = HashMap::new();
    counts.insert(last, 1);

    for (i, r) in ratings.iter().enumerate() {
        let count = counts.remove(&last).unwrap();
        let end = (i + 3).min(ratings.len());

        ratings[i..end]
            .iter()
            .filter(|r| **r <= last + 3)
            .for_each(|r| *counts.entry(*r).or_insert(0) += count);

        last = *r;
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
