use common::read_main;

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let line = line.as_bytes();
            if line.iter().filter(|c| b"aeiou".contains(c)).take(3).count() < 3 {
                return false;
            }

            if line.windows(2).any(|pair| {
                [
                    b"ab".as_slice(),
                    b"cd".as_slice(),
                    b"pq".as_slice(),
                    b"xy".as_slice(),
                ]
                .contains(&pair)
            }) {
                return false;
            }

            line.windows(2).any(|pair| pair[0] == pair[1])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";
        assert_eq!(solve(input), 2);
    }
}

read_main!();
