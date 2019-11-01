fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut difference = 4;

            let total_length = l.len();
            let mut unesacaped: String = l.replace(r#"\\"#, "");
            difference += total_length - unesacaped.len();

            let total_length = unesacaped.len();
            unesacaped = unesacaped.replace(r#"\""#, "");
            difference += total_length - unesacaped.len();

            let total_length = unesacaped.len();
            unesacaped = unesacaped.replace(r#"\x"#, "");
            difference += (total_length - unesacaped.len()) / 2;

            difference
        })
        .sum()
}

#[cfg(test)]
mod eight_b {
    use super::*;

    #[test]
    fn test() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(solve(input), 19);
    }
}

common::read_main!();
