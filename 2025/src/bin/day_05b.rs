fn solve(input: &str) -> usize {
    let (fresh_ranges, _) = input.trim().split_once("\n\n").unwrap();
    let mut fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            let min: u64 = min.parse().unwrap();
            let max: u64 = max.parse().unwrap();
            min..=max
        })
        .collect();

    fresh_ranges.sort_by_key(|r| std::cmp::Reverse(*r.start()));

    let Some(mut min_range) = fresh_ranges.pop() else {
        return 0;
    };

    let mut num_fresh = 0;
    while let Some(next_range) = fresh_ranges.pop() {
        if min_range.end() < next_range.start() {
            num_fresh += std::mem::replace(&mut min_range, next_range).count();
        } else {
            let end = std::cmp::max(*next_range.end(), *min_range.end());
            min_range = *min_range.start()..=end
        }
    }

    num_fresh + min_range.count()
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
        14
    );
}
