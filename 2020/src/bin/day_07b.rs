use regex::Regex;
use std::collections::HashMap;

fn solve(rules: &str) -> usize {
    let parent_regex = Regex::new(r"(?P<parent>.*) bags contain (?P<contents>.*)\.").unwrap();
    let contents_regex = Regex::new(r"(?P<amount>\d+) (?P<kind>.*) bag[s]?").unwrap();

    let rules = rules
        .trim()
        .lines()
        .filter_map(|rule| {
            let captures = parent_regex.captures(rule).unwrap();
            let parent = captures.name("parent").unwrap().as_str();
            let contents = captures.name("contents").unwrap().as_str();
            if contents == "no other bags" {
                return None;
            }

            let contents: Vec<_> = contents
                .split(',')
                .map(|c| {
                    let captures = contents_regex.captures(c).unwrap();
                    Content {
                        amount: captures.name("amount").unwrap().as_str().parse().unwrap(),
                        kind: captures.name("kind").unwrap().as_str(),
                    }
                })
                .collect();

            Some((parent, contents))
        })
        .collect();

    let rules = Rules(rules);

    bags_inside(&rules, "shiny gold")
}

fn bags_inside(rules: &Rules<'_>, bag: &str) -> usize {
    let contents = match rules.0.get(bag) {
        Some(c) => c,
        None => return 0,
    };

    contents
        .iter()
        .map(|c| c.amount * (bags_inside(rules, c.kind) + 1))
        .sum()
}

#[derive(Debug)]
struct Rules<'s>(HashMap<&'s str, Vec<Content<'s>>>);

#[derive(Debug)]
struct Content<'s> {
    amount: usize,
    kind: &'s str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve(input), 32);
    }

    #[test]
    fn example_two() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(solve(input), 126);
    }
}

common::read_main!();
