use common::read_main;

fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|pair| {
            let (first, second) = pair.split_once(',').unwrap();
            let first = {
                let (start, end) = first.split_once('-').unwrap();
                let start: u32 = start.parse().unwrap();
                let end: u32 = end.parse().unwrap();
                start..=end
            };
            let second = {
                let (start, end) = second.split_once('-').unwrap();
                let start: u32 = start.parse().unwrap();
                let end: u32 = end.parse().unwrap();
                start..=end
            };
            first.contains(second.start()) && first.contains(second.end())
                || second.contains(first.start()) && second.contains(first.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(solve(input), 2);
    }
}

read_main!();
