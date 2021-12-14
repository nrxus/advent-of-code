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

    let template = template.as_bytes();
    let last = *template.last().unwrap();
    let empty_counts: HashMap<&[u8], usize> = rules.keys().map(|&k| (k, 0)).collect();
    let mut pair_counts: HashMap<&[u8], usize> = empty_counts.clone();

    for pair in template.windows(2) {
        *pair_counts.get_mut(pair).unwrap() += 1;
    }

    for _ in 0..40 {
        pair_counts = pair_counts
            .into_iter()
            .fold(empty_counts.clone(), |mut counts, (k, v)| {
                let inserted = rules[k];
                *counts.get_mut([k[0], inserted].as_slice()).unwrap() += v;
                *counts.get_mut([inserted, k[1]].as_slice()).unwrap() += v;
                counts
            })
    }

    let mut counts: HashMap<u8, usize> =
        pair_counts
            .into_iter()
            .fold(HashMap::new(), |mut counts, (k, v)| {
                *counts.entry(k[0]).or_default() += v;
                counts
            });

    *counts.entry(last).or_default() += 1;

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
        assert_eq!(solve(input), 2188189693529);
    }
}

common::read_main!();
