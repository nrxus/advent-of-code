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

    let mut currents: Vec<_> = nodes.keys().filter(|k| k.ends_with('A')).copied().collect();
    let mut loop_indices = vec![None; currents.len()];

    for (i, direction) in directions.enumerate() {
        for (c_index, c) in currents.iter().enumerate() {
            if loop_indices[c_index] == None && c.ends_with('Z') {
                loop_indices[c_index] = Some(i);
            }
        }

        let found: Vec<_> = loop_indices.iter().flatten().copied().collect();
        if found.len() == loop_indices.len() {
            return lcm(found);
        }

        currents.iter_mut().for_each(|current| {
            let nexts = nodes.get(current).unwrap();
            *current = match direction {
                'L' => nexts.0,
                'R' => nexts.1,
                _ => unreachable!(),
            };
        })
    }

    unreachable!()
}

fn lcm(numbers: Vec<usize>) -> usize {
    numbers
        .into_iter()
        .reduce(|a, b| {
            let gcd = gcd(a, b);
            a / gcd * b
        })
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let remainder = a % b;
        gcd(b, remainder)
    }
}

common::read_main!();

#[test]
fn example_one() {
    let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    assert_eq!(solve(input), 6);
}
