use common::read_main;
use md5::Digest;

fn solve(input: &str) -> u32 {
    let input = input.trim();

    (1..)
        .find(|i| {
            let mut hasher = md5::Md5::new();
            hasher.update(format!("{input}{i}"));
            let result = hasher.finalize();
            result[0..2] == [0, 0] && result[2] < 0x10
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("abcdef"), 609043);
        assert_eq!(solve("pqrstuv"), 1048970);
    }
}

read_main!();
