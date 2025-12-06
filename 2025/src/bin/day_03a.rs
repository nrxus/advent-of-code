fn solve(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|bank| {
            let bank = bank.as_bytes();
            let mut max = [bank[0], bank[1]];
            for pair in bank.windows(2) {
                if pair[0] > max[0] {
                    max = [pair[0], pair[1]]
                } else if pair[1] > max[1] {
                    max[1] = pair[1]
                }
            }
            (max[0] - b'0') as u64 * 10 + (max[1] - b'0') as u64
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"987654321111111
811111111111119
234234234234278
818181911112111
"
        ),
        357
    );
}
