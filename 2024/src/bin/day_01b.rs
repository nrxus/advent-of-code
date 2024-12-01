use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    let mut left = HashMap::new();
    let mut right = HashMap::new();

    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
        .for_each(|mut tuple| {
            *left.entry(tuple.next().unwrap()).or_insert(0) += 1;
            *right.entry(tuple.next().unwrap()).or_insert(0) += 1;
        });

    left.into_iter()
        .filter_map(|(num, count)| {
            let right_count = right.remove(&num)?;
            Some(count * num * right_count)
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"3   4
4   3
2   5
1   3
3   9
3   3
"
        ),
        31
    );
}
