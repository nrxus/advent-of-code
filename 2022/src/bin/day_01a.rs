use common::read_main;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .split("\n\n")
        .map(|bag| bag.lines().map(|calories| calories.parse::<u32>().unwrap()))
        .map(|calories| calories.sum())
        .max()
        .unwrap_or_default()
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

        assert_eq!(solve(input), 24000);
    }
}

read_main!();
