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
    let has_strict_double = has_strict_pair(&password);
    let decreases = password.windows(2).any(|window| window[0] > window[1]);
    has_strict_double && !decreases
}

fn has_strict_pair(password: &Vec<u8>) -> bool {
    let mut repeat_count = 0;
    for window in password.windows(2) {
        let is_a_pair = window[0] == window[1];
        if repeat_count == 1 && !is_a_pair {
            return true;
        } else if is_a_pair {
            repeat_count += 1;
        } else {
            repeat_count = 0;
        }
    }
    repeat_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(matches_password(112233));
        assert!(matches_password(111122));

        assert!(!matches_password(123444));
    }
}

common::read_main!();
