use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

fn solve(input: &str) -> usize {
    let boxes: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|c| c.parse::<u32>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            let z = coords.next().unwrap();
            (x, y, z)
        })
        .collect();

    let mut distances = BinaryHeap::with_capacity((boxes.len() * (boxes.len() - 1)) / 2);

    for (i, &a) in boxes.iter().enumerate() {
        let boxes = boxes[i + 1..]
            .iter()
            .map(move |&b| {
                let distance = (((a.0.abs_diff(b.0) as u64).pow(2)
                    + (a.1.abs_diff(b.1) as u64).pow(2)
                    + (a.2.abs_diff(b.2) as u64).pow(2)) as f64)
                    .sqrt();
                BoxPair { a, b, distance }
            })
            .map(cmp::Reverse);
        distances.extend(boxes);
    }

    let mut circuits: Vec<_> = boxes
        .into_iter()
        .map(|b| HashSet::<_>::from_iter([b]))
        .collect();

    for _ in 0..NUM_CONNECTIONS {
        let cmp::Reverse(pair) = distances.pop().unwrap();
        let circuit_with_a = circuits
            .iter()
            .enumerate()
            .find_map(|(i, circuit)| {
                if circuit.contains(&pair.a) {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();
        let circuit_with_b = circuits
            .iter()
            .enumerate()
            .find_map(|(i, circuit)| {
                if circuit.contains(&pair.b) {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();
        if circuit_with_b != circuit_with_a {
            let max = cmp::max(circuit_with_a, circuit_with_b);
            let min = cmp::min(circuit_with_a, circuit_with_b);
            let circuit_with_max = circuits.swap_remove(max);
            circuits[min].extend(circuit_with_max);
        }
    }

    circuits.sort_by_key(|c| c.len());
    circuits
        .into_iter()
        .rev()
        .take(3)
        .map(|c| c.len())
        .product()
}

#[derive(Debug, PartialEq, PartialOrd)]
struct BoxPair {
    distance: f64,
    a: (u32, u32, u32),
    b: (u32, u32, u32),
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for BoxPair {}

#[cfg(not(test))]
const NUM_CONNECTIONS: usize = 1000;

#[cfg(test)]
const NUM_CONNECTIONS: usize = 10;

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        ),
        40
    );
}
