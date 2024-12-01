fn solve(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];

    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
        .for_each(|mut tuple| {
            left.push(tuple.next().unwrap());
            right.push(tuple.next().unwrap());
        });

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
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
        11
    );
}
