use std::collections::{hash_map, HashMap, HashSet};

fn solve(input: &str) -> usize {
    let (towels, needed) = input.trim().split_once("\n\n").unwrap();
    let towels: Vec<_> = towels.split(',').map(|t| t.trim()).collect();

    needed
        .lines()
        .map(|needed| {
            let mut frontier = vec![needed];
            let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
            while let Some(next) = frontier.pop() {
                if next.is_empty() {
                    continue;
                }

                let neighbors = towels.iter().filter_map(|towel| {
                    let stripped = next.strip_prefix(towel)?;
                    match graph.entry(stripped) {
                        hash_map::Entry::Occupied(o) => {
                            o.into_mut().insert(next);
                            None
                        }
                        hash_map::Entry::Vacant(v) => {
                            v.insert(HashSet::new()).insert(next);
                            Some(stripped)
                        }
                    }
                });

                frontier.extend(neighbors)
            }

            let Some(solutions) = graph.remove("") else {
                return 0;
            };

            let mut sums = HashMap::new();
            solutions
                .into_iter()
                .map(|next| {
                    if let Some(s) = sums.get(next).copied() {
                        s
                    } else {
                        let s = sum_leaves(next, &mut graph, &mut sums);
                        sums.insert(next, s);
                        s
                    }
                })
                .sum()
        })
        .sum()
}

fn sum_leaves<'s>(
    branch: &'s str,
    graph: &mut HashMap<&'s str, HashSet<&'s str>>,
    sums: &mut HashMap<&'s str, usize>,
) -> usize {
    let Some(nexts) = graph.remove(branch) else {
        return 1;
    };

    let mut sum = 0;
    for next in nexts {
        let s = if let Some(s) = sums.get(next).copied() {
            s
        } else {
            let s = sum_leaves(next, graph, sums);
            sums.insert(next, s);
            s
        };
        sum += s;
    }
    sum
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        ),
        16
    );
}
