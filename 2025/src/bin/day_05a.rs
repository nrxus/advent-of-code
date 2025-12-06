fn solve(input: &str) -> usize {
    let (fresh_ranges, ingredient_ids) = input.trim().split_once("\n\n").unwrap();
    let fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            let min: u64 = min.parse().unwrap();
            let max: u64 = max.parse().unwrap();
            min..=max
        })
        .collect();

    ingredient_ids
        .lines()
        .filter(|id| {
            let id: u64 = id.parse().unwrap();
            fresh_ranges.iter().any(|r| r.contains(&id))
        })
        .count()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        ),
        3
    );
}
