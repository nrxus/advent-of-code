fn solve(input: &str) -> usize {
    let range: Vec<_> = input
        .trim()
        .split("-")
        .map(|r| r.parse::<u32>().unwrap())
        .collect();
    let lower_range = range[0];
    let upper_range = range[1];
    (lower_range..=upper_range)
        .filter(|&p| matches_password(p))
        .count()
}

// ignores range and six-digit requirement
fn matches_password(password: u32) -> bool {
    let password = password.to_string().into_bytes();
    let has_double = password.windows(2).any(|window| window[0] == window[1]);
    let decreases = password.windows(2).any(|window| window[0] > window[1]);
    has_double && !decreases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(matches_password(111111));
        assert!(!matches_password(223450));
        assert!(!matches_password(123789));
    }
}

common::read_main!();
