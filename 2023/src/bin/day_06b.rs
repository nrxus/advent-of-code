fn solve(input: &str) -> usize {
    let mut input = input.trim().lines().map(|l| {
        let (_, numbers) = l.split_once(':').unwrap();
        numbers
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });

    let total_time = input.next().unwrap();
    let record = input.next().unwrap();

    (1..total_time)
        .map(|t| t * (total_time - t))
        .skip_while(|x| x <= &record)
        .take_while(|x| x > &record)
        .count()
}

common::read_main!();

#[test]
fn example() {
    let input = r"Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(solve(input), 71503);
}
