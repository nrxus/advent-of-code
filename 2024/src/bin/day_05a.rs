use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> u32 {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();
    let less_than = rules
        .lines()
        .map(|rule| rule.split_once('|').unwrap())
        .fold(HashMap::new(), |mut less_than, (left, right)| {
            less_than
                .entry(left)
                .or_insert(HashSet::new())
                .insert(right);
            less_than
        });

    updates
        .lines()
        .filter_map(|update| {
            let pages: Vec<_> = update.split(',').collect();
            let is_sorted = pages.is_sorted_by(|a, b| {
                if let Some(less_than) = less_than.get(a) {
                    if less_than.contains(b) {
                        return true;
                    }
                }
                if let Some(less_than) = less_than.get(b) {
                    if less_than.contains(a) {
                        return false;
                    }
                }
                panic!("AH NO");
            });

            if is_sorted {
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .map(|page| page.parse::<u32>().unwrap())
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
        ),
        143
    );
}
