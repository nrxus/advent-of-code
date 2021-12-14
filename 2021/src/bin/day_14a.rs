use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let (template, rules) = input.trim().split_once("\n\n").unwrap();
    let rules: HashMap<&[u8], u8> = rules
        .lines()
        .map(|line| {
            let (pair, inserted) = line.split_once("->").unwrap();
            let pair = pair.trim().as_bytes();
            let inserted = inserted.trim().as_bytes();

            (pair, inserted[0])
        })
        .collect();

    let mut template = template.as_bytes().to_owned();

    for _ in 0..10 {
        template = template
            .windows(2)
            .flat_map(|pair| [pair[0], rules[pair]])
            .chain(template.last().copied())
            .collect();
    }

    let counts: HashMap<u8, usize> =
        template
            .into_iter()
            .fold(HashMap::new(), |mut counts, byte| {
                *counts.entry(byte).or_default() += 1;
                counts
            });

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(solve(input), 1588);
    }
}

common::read_main!();
