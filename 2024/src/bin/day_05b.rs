use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

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

    let page_cmp = move |a: &str, b: &str| {
        if let Some(less_than) = less_than.get(a) {
            if less_than.contains(b) {
                return Ordering::Less;
            }
        }
        if let Some(less_than) = less_than.get(b) {
            if less_than.contains(a) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    };

    updates
        .lines()
        .filter_map(|update| {
            let mut pages: Vec<_> = update.split(',').collect();
            let is_sorted = pages.is_sorted_by(|a, b| (page_cmp(a, b).is_lt()));

            if is_sorted {
                return None;
            }

            let mid = pages.len() / 2;
            let (_, mid, _) = pages.select_nth_unstable_by(mid, |a, b| page_cmp(a, b));
            Some(*mid)
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
        123
    );
}
