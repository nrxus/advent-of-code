fn solve(input: &str) -> u32 {
    let mut calories_per_bag: Vec<u32> = input
        .trim()
        .split("\n\n")
        .map(|bag| bag.lines().map(|calories| calories.parse::<u32>().unwrap()))
        .map(|calories| calories.sum())
        .collect();

    calories_per_bag.sort();
    calories_per_bag.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        assert_eq!(solve(input), 45000);
    }
}

common::read_main!();
