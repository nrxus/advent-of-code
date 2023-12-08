use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let (directions, nodes) = input.trim().split_once("\n\n").unwrap();
    let directions = std::iter::repeat(directions.trim()).flat_map(|s| s.chars());

    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(|l| {
            let (node, next) = l.split_once('=').unwrap();
            let (left, right) = next.split_once(',').unwrap();
            let left = &left.trim()[1..];
            let right = right.trim();
            let right = &right[0..right.len() - 1];
            (node.trim(), (left, right))
        })
        .collect();

    let mut current = "AAA";

    for (i, direction) in directions.enumerate() {
        if current == "ZZZ" {
            return i;
        }
        let nexts = nodes.get(current).unwrap();
        current = match direction {
            'L' => nexts.0,
            'R' => nexts.1,
            _ => unreachable!(),
        };
    }

    unreachable!()
}

common::read_main!();

#[test]
fn example_one() {
    let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(solve(input), 2);
}

#[test]
fn example_two() {
    let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(solve(input), 6);
}
