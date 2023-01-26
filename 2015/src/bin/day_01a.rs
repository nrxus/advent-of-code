use common::read_main;

fn solve(input: &str) -> i16 {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => panic!("unexpected {c}"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("(())"), 0);
        assert_eq!(solve("()()"), 0);
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
        assert_eq!(solve("))((((("), 3);
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }
}

read_main!();
