fn solve(input: &str) -> u64 {
    let mut ranges: Vec<_> = input
        .trim()
        .split(',')
        .map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            let low: u64 = low.parse().unwrap();
            let high: u64 = high.parse().unwrap();
            low..=high
        })
        .collect();

    ranges.sort_by_key(|r| std::cmp::Reverse(*r.start()));
    let mut range = ranges.pop().unwrap();

    let mut sum: u64 = 0;
    'outer: for bogus_id in (1u64..).map(|i| format!("{i}{i}").parse::<u64>().unwrap()) {
        while bogus_id > *range.end() {
            let Some(next_range) = ranges.pop() else {
                break 'outer;
            };
            range = next_range;
        }

        if range.contains(&bogus_id) {
            sum += bogus_id;
            continue;
        }
    }

    sum
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        ),
        1227775554
    );
}
