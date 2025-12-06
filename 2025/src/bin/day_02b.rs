use fancy_regex::Regex;

fn solve(input: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\1{1,}$").unwrap();
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            let low: u64 = low.parse().unwrap();
            let high: u64 = high.parse().unwrap();
            low..=high
        })
        .filter(|n| {
            let strn = format!("{n}");
            re.is_match(&strn).unwrap()
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        ),
        4174379265
    );
}
