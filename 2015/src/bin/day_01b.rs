use common::read_main;

fn solve(input: &str) -> usize {
    input
        .trim()
        .chars()
        .scan(0, |state, c| {
            match c {
                '(' => {
                    *state += 1;
                }
                ')' => {
                    *state -= 1;
                }
                c => panic!("unexpected {c}"),
            };
            Some(*state)
        })
        .position(i16::is_negative)
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(")"), 1);
        assert_eq!(solve("()())"), 5);
    }
}

read_main!();
