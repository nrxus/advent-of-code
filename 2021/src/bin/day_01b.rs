fn solve(input: &str) -> usize {
    let depths: Vec<u16> = input
        .trim()
        .lines()
        .map(|depth| depth.parse().unwrap())
        .collect();

    let sliding_depth: Vec<u16> = depths.windows(3).map(|trio| trio.iter().sum()).collect();

    sliding_depth
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"199
200
208
210
200
207
240
269
260
263";
        assert_eq!(solve(input), 5);
    }
}

common::read_main!();
