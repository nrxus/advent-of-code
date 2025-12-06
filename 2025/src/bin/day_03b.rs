fn solve(input: &str) -> u64 {
    input.trim().lines().map(largest_voltage::<12>).sum()
}

fn largest_voltage<const N: usize>(bank: &str) -> u64 {
    assert!(bank.len() >= N);
    let bank = bank.as_bytes();
    let mut max = [0u8; N];
    max.copy_from_slice(&bank[0..N]);

    for set in bank[1..].windows(N) {
        for (idx, digit) in set.iter().enumerate() {
            if *digit > max[idx] {
                max[idx..N].copy_from_slice(&set[idx..N]);
                break;
            }
        }
    }

    max.iter()
        .rev()
        .enumerate()
        .map(|(idx, digit)| (digit - b'0') as u64 * 10_u64.pow(idx as u32))
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
        3121910778619
    );
}
