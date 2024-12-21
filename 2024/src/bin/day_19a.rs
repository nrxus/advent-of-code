use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let (towels, needed) = input.trim().split_once("\n\n").unwrap();
    let towels: Vec<_> = towels.split(',').map(|t| t.trim()).collect();

    needed
        .lines()
        .filter(|&needed| {
            let mut frontier = vec![needed];
            let mut explored = HashSet::new();
            while let Some(next) = frontier.pop() {
                if next.is_empty() {
                    return true;
                }

                let neighbors = towels
                    .iter()
                    .filter_map(|towel| next.strip_prefix(towel))
                    .filter(|n| explored.insert(*n));

                frontier.extend(neighbors)
            }

            false
        })
        .count()
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
        6
    );
}
