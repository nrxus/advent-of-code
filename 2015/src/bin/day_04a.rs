fn solve(input: &str) -> usize {
    (0..)
        .map(|i| format!("{}{}", input.trim(), i))
        .map(|data| md5::compute(data))
        .take_while(|hash| hash[0] != 0 || hash[1] != 0 || hash[2] > 15)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve("abcdef"), 609043);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve("pqrstuv"), 1048970);
    }
}

common::read_main!();
