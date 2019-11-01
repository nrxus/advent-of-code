fn solve(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unexpected input"),
        })
        .sum()
}

common::read_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(solve("(())"), 0);
        assert_eq!(solve("()()"), 0);
    }

    #[test]
    fn test_three() {
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
        assert_eq!(solve("))((((("), 3);
    }

    #[test]
    fn test_neg_one() {
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
    }

    #[test]
    fn test_neg_three() {
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }
}
