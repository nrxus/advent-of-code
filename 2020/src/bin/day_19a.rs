use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(messages: &str) -> usize {
    let line_regex = Regex::new(r"(?P<idx>\d+): (?P<rule>.*)").unwrap();

    let mut messages = messages.split("\n\n");
    let rules = messages.next().unwrap();
    let rules: HashMap<usize, _> = rules
        .lines()
        .map(|r| {
            let captures = line_regex.captures(r).unwrap();
            let index: usize = captures.name("idx").unwrap().as_str().parse().unwrap();
            let rule: Rule = captures.name("rule").unwrap().as_str().parse().unwrap();
            (index, rule)
        })
        .collect();

    let mut possibilities: HashSet<String> = HashSet::new();
    let mut frontier = vec![Node {
        unnested: vec![0],
        string: "".to_string(),
    }];

    while let Some(mut node) = frontier.pop() {
        let rule_idx = match node.unnested.pop() {
            Some(i) => i,
            None => {
                possibilities.insert(node.string);
                continue;
            }
        };

        match &rules[&rule_idx] {
            Rule::Char(c) => {
                node.string.push(*c);
                frontier.push(node);
            }
            Rule::Joined(joined) => {
                node.unnested.extend(joined);
                frontier.push(node);
            }
            Rule::Or(a, b) => {
                let mut other = node.clone();
                node.unnested.extend(a);
                other.unnested.extend(b);
                frontier.push(node);
                frontier.push(other);
            }
        }
    }

    let messages = messages.next().unwrap();
    messages
        .lines()
        .filter(|m| possibilities.contains(*m))
        .count()
}

fn parse_joined(raw: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    raw.split_whitespace().map(|r| r.parse()).rev().collect()
}

#[derive(Clone, Debug)]
struct Node {
    unnested: Vec<usize>,
    string: String,
}

#[derive(Debug)]
enum Rule {
    Or(Vec<usize>, Vec<usize>),
    Joined(Vec<usize>),
    Char(char),
}

impl std::str::FromStr for Rule {
    type Err = std::num::ParseIntError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CHAR_REGEX: Regex = Regex::new(r#""(?P<char>.*)""#).unwrap();
            static ref OR_REGEX: Regex = Regex::new(r"(?P<a>.*) \| (?P<b>.*)").unwrap();
        }

        // "a"
        if let Some(captures) = CHAR_REGEX.captures(raw) {
            return Ok(Rule::Char(
                captures
                    .name("char")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap(),
            ));
        }

        // 1 2 | 3 4
        if let Some(captures) = OR_REGEX.captures(raw) {
            let a = captures.name("a").unwrap().as_str();
            let b = captures.name("b").unwrap().as_str();

            return Ok(Rule::Or(parse_joined(a)?, parse_joined(b)?));
        }

        // 1 2 3 4
        Ok(Rule::Joined(parse_joined(raw)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        assert_eq!(solve(input), 2);
    }
}

common::read_main!();
