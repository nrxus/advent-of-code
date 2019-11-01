fn solve(input: &str) -> usize {
    let naughty_substrings = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let no_banned_strs = |l: &&str| !naughty_substrings.iter().any(|n| l.contains(n));
    let at_least_three_vowels = |l: &&str| l.chars().filter(|c| vowels.contains(c)).count() >= 3;
    let duplicate_letter = |l: &&str| {
        l.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w[0] == w[1])
    };

    input
        .lines()
        .filter(no_banned_strs)
        .filter(at_least_three_vowels)
        .filter(duplicate_letter)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singles() {
        assert_eq!(solve("ugknbfddgicrmopn"), 1);
        assert_eq!(solve("aaa"), 1);
        assert_eq!(solve("jchzalrnumimnmhp"), 0);
        assert_eq!(solve("haegwjzuvuyypxyu"), 0);
        assert_eq!(solve("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_many() {
        let input = r"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";
        assert_eq!(solve(input), 2);
    }
}

common::read_main!();
