mod polymer_len;

use self::polymer_len::polymer_len;

fn solve(input: &str) -> usize {
    let input = input.trim();

    (b'a'..=b'z')
        .map(|e| {
            input
                .bytes()
                .filter(|c| c.to_ascii_lowercase() != e)
                .collect::<Vec<_>>()
        })
        .map(|s| polymer_len(&s))
        .min()
        .unwrap()
}

common::read_main!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "dabAcCaCBAcCcaDA\n";
        assert_eq!(solve(input), 4);
    }
}
