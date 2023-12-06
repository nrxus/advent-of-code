fn solve(input: &str) -> usize {
    let mut input = input.trim().lines().map(|l| {
        let (_, numbers) = l.split_once(':').unwrap();
        numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
    });
    let times = input.next().unwrap();
    let distances = input.next().unwrap();
    times
        .zip(distances)
        .map(|(total_time, record)| {
            (1..total_time)
                .map(|t| t * (total_time - t))
                .skip_while(|x| x <= &record)
                .take_while(|x| x > &record)
                .count()
        })
        .product()
}

common::read_main!();

#[test]
fn example() {
    let input = r"Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(solve(input), 288);
}
