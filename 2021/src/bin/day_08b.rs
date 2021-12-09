use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let (signal, out) = line.split_once('|').unwrap();
            let decoded = decode_signal(signal);
            let out: String = out
                .split_whitespace()
                .map(|out| {
                    // assume ascii
                    let mut out: Vec<_> = out.bytes().collect();
                    out.sort_unstable();
                    let out = String::from_utf8(out).unwrap();
                    decoded[&out]
                })
                .collect();
            out.parse::<u32>().unwrap()
        })
        .sum()
}

fn decode_signal(signal: &str) -> HashMap<String, char> {
    let mut signal: Vec<&str> = signal.split_whitespace().collect();

    let one = signal.iter().position(|s| s.len() == 2).unwrap();
    let one: HashSet<_> = signal.swap_remove(one).chars().collect();

    let four = signal.iter().position(|s| s.len() == 4).unwrap();
    let four: HashSet<_> = signal.swap_remove(four).chars().collect();

    let seven = signal.iter().position(|s| s.len() == 3).unwrap();
    let seven: HashSet<_> = signal.swap_remove(seven).chars().collect();

    let eight = signal.iter().position(|s| s.len() == 7).unwrap();
    let eight: HashSet<_> = signal.swap_remove(eight).chars().collect();

    let six = signal
        .iter()
        .position(|s| s.len() == 6 && s.chars().filter(|c| one.contains(c)).count() == 1)
        .unwrap();
    let six: HashSet<_> = signal.swap_remove(six).chars().collect();

    let nine = signal
        .iter()
        .position(|s| s.len() == 6 && s.chars().filter(|c| four.contains(c)).count() == 4)
        .unwrap();
    let nine: HashSet<_> = signal.swap_remove(nine).chars().collect();

    let zero = signal.iter().position(|s| s.len() == 6).unwrap();
    let zero: HashSet<_> = signal.swap_remove(zero).chars().collect();

    let three = signal
        .iter()
        .position(|s| s.chars().filter(|c| one.contains(c)).count() == 2)
        .unwrap();
    let three: HashSet<_> = signal.swap_remove(three).chars().collect();

    let five = signal
        .iter()
        .position(|s| s.chars().filter(|c| nine.contains(c)).count() == 5)
        .unwrap();
    let five: HashSet<_> = signal.swap_remove(five).chars().collect();

    let two = signal.pop().unwrap().chars().collect();

    [
        (to_sorted_string(one), '1'),
        (to_sorted_string(two), '2'),
        (to_sorted_string(three), '3'),
        (to_sorted_string(four), '4'),
        (to_sorted_string(five), '5'),
        (to_sorted_string(six), '6'),
        (to_sorted_string(seven), '7'),
        (to_sorted_string(eight), '8'),
        (to_sorted_string(nine), '9'),
        (to_sorted_string(zero), '0'),
    ]
    .into_iter()
    .collect()
}

fn to_sorted_string(chars: HashSet<char>) -> String {
    let mut chars: Vec<_> = chars.into_iter().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(solve(input), 61229);
    }

    #[test]
    fn simple() {
        let input =
            r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solve(input), 5353);
    }
}

common::read_main!();
