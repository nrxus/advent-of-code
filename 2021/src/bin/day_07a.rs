fn solve(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    input.sort_unstable();

    let median = if input.len() % 2 == 1 {
        input[input.len() / 2]
    } else {
        let double_median = input[input.len() / 2 - 1] + input[input.len() / 2];
        if double_median % 2 == 0 {
            double_median / 2
        } else {
            todo!()
        }
    };

    input
        .into_iter()
        .map(|d| if d > median { d - median } else { median - d })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve(input), 37);
    }
}

common::read_main!();
