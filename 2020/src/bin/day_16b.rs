use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

fn solve(input: &str) -> u64 {
    let ticket = decipher(input);

    ticket
        .into_iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| v as u64)
        .product()
}

fn decipher(input: &str) -> HashMap<&str, u32> {
    let mut input = input.trim().split("\n\n");
    let rules = input.next().unwrap();
    let mine: Vec<_> = input
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    let nearby = input.next().unwrap();

    let constraint_regex = Regex::new(
        r"(?P<name>.*): (?P<min_one>\d+)-(?P<max_one>\d+) or (?P<min_two>\d+)-(?P<max_two>\d+)",
    )
    .unwrap();

    let fields: Vec<_> = rules
        .lines()
        .map(|rule| {
            let captures = constraint_regex.captures(rule).unwrap();
            let name = captures.name("name").unwrap().as_str();
            let min_one = captures.name("min_one").unwrap().as_str().parse().unwrap();
            let max_one = captures.name("max_one").unwrap().as_str().parse().unwrap();
            let min_two = captures.name("min_two").unwrap().as_str().parse().unwrap();
            let max_two = captures.name("max_two").unwrap().as_str().parse().unwrap();

            Field {
                name,
                first: min_one..=max_one,
                second: min_two..=max_two,
            }
        })
        .collect();

    // in the beginning the possibilities are endless!
    // (every "index" could be any of the fields)
    let mut possibilities: HashMap<_, _> = (0..fields.len()).map(|i| (i, fields.clone())).collect();

    // but then reality strikes us down and we see that we actually only fit in some places
    // (remove invalid tickets + first-pass narrow down possibilities)
    // (filter on whether a number fits a pattern without any regard for other tickets)
    nearby
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()))
        .for_each(|ticket| {
            for (i, value) in ticket.enumerate() {
                let possibilities = possibilities.get_mut(&i).unwrap();
                // if it doesn't match any of the possibilities
                // skip this invalid ticket
                if possibilities.iter().all(|field| !field.matches(value)) {
                    return;
                }
                possibilities.retain(|field| field.matches(value))
            }
        });

    let mut deciphered = HashMap::new();

    // but then even those places we did fit n were already taken
    // by those that fit nowhere else
    // (get the field with only a single option as we have found a match)
    // (a min-heap couldn't work because it has to "live-update" elements already inside)
    while let Some((i, _)) = possibilities.iter_mut().min_by_key(|(_, f)| f.len()) {
        let i = *i; // stop the borrow early so the borrow checker doesn't stab us
        let fields = possibilities.remove(&i).unwrap();
        debug_assert!(fields.len() == 1);

        let name = fields[0].name;
        deciphered.insert(name, mine[i]);

        possibilities.values_mut().for_each(|p| {
            p.retain(|f| f.name != name);
        })
    }

    deciphered
}

#[derive(Debug, Clone)]
struct Field<'n> {
    name: &'n str,
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl Field<'_> {
    fn matches(&self, n: u32) -> bool {
        self.first.contains(&n) || self.second.contains(&n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let expected = [("class", 12), ("row", 11), ("seat", 13)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(decipher(input), expected);
    }
}

common::read_main!();
