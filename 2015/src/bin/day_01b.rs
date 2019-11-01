fn solve(input: &str) -> usize {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unexpected input"),
        })
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .position(|floor| floor == -1)
        .unwrap()
        + 1
}

common::read_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(solve(")"), 1);
    }

    #[test]
    fn test_middle() {
        assert_eq!(solve("()())"), 5);
    }
}
