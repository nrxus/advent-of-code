use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|card| {
            let (_, card) = card.split_once(':').unwrap();
            let (winners, haves) = card.trim().split_once('|').unwrap();
            let winners: HashSet<_> = winners.trim().split_whitespace().collect();
            let haves: HashSet<_> = haves.trim().split_whitespace().collect();
            let matches = winners.intersection(&haves).count();
            let Some(pow) = matches.checked_sub(1) else {
                return 0;
            };
            2_u32.pow(pow as u32)
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(solve(input), 13);
}
