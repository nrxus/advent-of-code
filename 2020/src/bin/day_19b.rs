use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(messages: &str) -> usize {
    let line_regex = Regex::new(r"(?P<idx>\d+): (?P<rule>.*)").unwrap();

    let mut messages = messages.split("\n\n");
    let rules = messages.next().unwrap();
    let rules: Rules = rules
        .lines()
        .map(|r| {
            let captures = line_regex.captures(r).unwrap();
            let index: usize = captures.name("idx").unwrap().as_str().parse().unwrap();
            let rule: Rule = captures.name("rule").unwrap().as_str().parse().unwrap();
            (index, rule)
        })
        .collect();

    let r31 = expand(&rules, 31);
    let r42 = expand(&rules, 42);
    let chunk_len = r31.iter().next().unwrap().len();

    // 0: 8 11
    // 0: (42 | 42 8) (42 31 | 42 11 31)
    // 0: (42)+ 42+ 31+ (the number of 42+ should match number of 31+)
    messages
        .next()
        .unwrap()
        .lines()
        .filter(|l| {
            let mut unmatched = *l;

            let mut matches_start = 0_usize;
            while unmatched.len() >= chunk_len && r42.contains(&unmatched[..chunk_len]) {
                unmatched = &unmatched[chunk_len..];
                matches_start +=1;
            }

            if matches_start < 2 {
                return false;
            }

            let mut matches_end = 0_usize;
            while !unmatched.is_empty() && r31.contains(&unmatched[..chunk_len]) {
                unmatched = &unmatched[chunk_len..];
                matches_end +=1;
            }

            unmatched.is_empty() && matches_end > 0 && matches_end < matches_start
        })
        .count()
}

type Rules = HashMap<usize, Rule>;

fn expand(rules: &Rules, idx: usize) -> HashSet<String> {
    let mut possibilities: HashSet<String> = HashSet::new();
    let mut frontier = vec![Node {
        unnested: vec![idx],
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

    possibilities
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
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn example_two() {
        let input = r#"0: 8 11
1: 54 47
2: 47 25
3: 14 32
4: 32 9
5: 48 32 | 4 47
6: 50 47
7: 21 32
8: 42
9: 2 47
10: 47 25
11: 42 31
12: 32 20
13: 47 33 | 32 16
14: 43 32
15: 32 45
16: 47 35
17: 47 32
18: 7 47
19: 3 47 | 13 32
20: 21 32
21: 47 47
22: 26 47 | 27 32
23: 55 47
24: 25 47
25: 47 47 | 32 47
26: 47 53
27: 6 47
28: 15 32
29: 47 36
30: 47 25
31: 22 32 | 40 47
32: "b"
33: 12 32
34: 49 47
35: 24 32
36: 32 18
37: 47 30
38: 32 47
39: 23 32
40: 47 29 | 32 5
41: 47 10
42: 19 32 | 52 47
43: 32 46
44: 39 47
45: 47 47
46: 32 17
47: "a"
48: 32 28
49: 38 47
50: 51 32
51: 17 47
52: 47 1
53: 32 34 | 47 44
54: 41 47 | 37 32
55: 47

aaabaaaabbabbbabbabaabbbabaabbbbaaaaabaaaabaabaaababaabbabbaaaabbbbaabbabbabaaaaaaaabaab"#;
        assert_eq!(solve(input), 0);
    }
}

common::read_main!();
