mod polymer_len;

use self::polymer_len::polymer_len;

fn solve(input: &str) -> usize {
    polymer_len(input.trim().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn test_2() {
        let input = "cBaAbdDB";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_3() {
        let input = "ZzNnMYytTtfFTINnkKimYUOouyPpvVokKZzqQQqTtMmmIiMjJryYvVRztJjiIfFyGbBgYTgGLfFHhlsysSYoOsSiIfpPFcCrRVvbeEuUAXx";
        assert_eq!(solve(input), 5);
    }

    #[test]
    fn test_4() {
        let input = "aBbA\n";
        assert_eq!(solve(input), 0);
    }
}

common::read_main!();
