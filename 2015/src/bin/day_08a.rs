fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut difference = 2;

            let total_length = l.len();
            let mut unesacaped: String = l.replace(r#"\\"#, "");
            difference += (total_length - unesacaped.len()) / 2;

            let total_length = unesacaped.len();
            unesacaped = unesacaped.replace(r#"\""#, "");
            difference += (total_length - unesacaped.len()) / 2;

            let total_length = unesacaped.len();
            unesacaped = unesacaped.replace(r#"\x"#, "");
            difference += (total_length - unesacaped.len()) / 2 * 3;

            difference
        })
        .sum()
}

#[cfg(test)]
mod eight_a {
    use super::*;

    #[test]
    fn test() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(solve(input), 12);
    }
}

common::read_main!();
