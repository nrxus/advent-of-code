fn solve(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
        .map(|sequence| {
            let sequence: Vec<_> = sequence.collect();
            extrapolate_sequence(&sequence)
        })
        .sum()
}

fn extrapolate_sequence(sequence: &[i32]) -> i32 {
    let mut all_zeros = true;

    let next: Vec<_> = sequence
        .windows(2)
        .map(|pair| {
            let next = pair[1] - pair[0];
            all_zeros = all_zeros && next == 0;
            next
        })
        .collect();

    if all_zeros {
        return sequence.last().copied().unwrap_or(0);
    }

    extrapolate_sequence(&next) + sequence.last().unwrap()
}

common::read_main!();

#[test]
fn example_one() {
    let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(solve(input), 114);
}
