use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let stones: HashMap<u64, usize> = input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut stones, stone| {
            *stones.entry(stone).or_insert(0) += 1;
            stones
        });

    let stones = (0..25).fold(stones, |stones, _| {
        stones
            .into_iter()
            .fold(HashMap::new(), |mut stones, (stone, count)| {
                if stone == 0 {
                    *stones.entry(1).or_insert(0) += count;
                } else {
                    let stone_str = format!("{stone}");
                    if stone_str.len() % 2 == 0 {
                        let (left, right) = stone_str.split_at(stone_str.len() / 2);
                        *stones.entry(left.parse().unwrap()).or_insert(0) += count;
                        *stones.entry(right.parse().unwrap()).or_insert(0) += count;
                    } else {
                        *stones.entry(stone * 2024).or_insert(0) += count;
                    }
                }

                stones
            })
    });

    stones.into_values().sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(solve(r"125 17"), 55312);
}
