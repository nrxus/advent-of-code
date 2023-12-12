use std::collections::HashMap;

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let (readings, counts) = l.split_once(' ').unwrap();
            let counts: Vec<_> = counts
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            let len = counts.len() * 5;
            let counts: Vec<_> = counts.into_iter().cycle().take(len).collect();

            let readings = vec![readings; 5];
            let readings = readings.join("?");

            permutate(&readings, &counts)
        })
        .sum()
}

fn permutate(line: &str, desired: &[usize]) -> usize {
    let mut universes: HashMap<(Vec<usize>, Option<usize>), usize> =
        HashMap::from_iter([((vec![], None), 1)]);

    for c in line.chars() {
        match c {
            '.' => {
                universes = collect_with_count(universes.into_iter().filter_map(
                    |((mut so_far, mut latest), count)| {
                        if let Some(latest) = latest.take() {
                            desired.get(so_far.len()).filter(|d| **d == latest)?;
                            so_far.push(latest)
                        }
                        Some(((so_far, latest), count))
                    },
                ));
            }
            '#' => {
                universes =
                    collect_with_count(universes.into_iter().filter_map(|((so_far, l), count)| {
                        let next = l.unwrap_or(0) + 1;
                        desired.get(so_far.len()).filter(|d| **d >= next)?;
                        Some(((so_far, Some(next)), count))
                    }))
            }
            '?' => {
                let alt_universes = universes.clone().into_iter().filter_map(
                    |((mut so_far, mut latest), count)| {
                        if let Some(latest) = latest.take() {
                            desired.get(so_far.len()).filter(|d| **d == latest)?;
                            so_far.push(latest)
                        }
                        Some(((so_far, latest), count))
                    },
                );

                universes = collect_with_count(
                    universes
                        .into_iter()
                        .filter_map(|((so_far, l), count)| {
                            let next = l.unwrap_or(0) + 1;
                            desired.get(so_far.len()).filter(|d| **d >= next)?;
                            Some(((so_far, Some(next)), count))
                        })
                        .chain(alt_universes),
                );
            }
            _ => unreachable!(),
        };
    }

    universes = collect_with_count(universes.into_iter().filter_map(
        |((mut so_far, mut latest), count)| {
            if let Some(latest) = latest.take() {
                desired.get(so_far.len()).filter(|d| **d == latest)?;
                so_far.push(latest)
            }
            Some(((so_far, latest), count))
        },
    ));

    universes
        .into_iter()
        .filter_map(|((u, _), c)| if u == desired { Some(c) } else { None })
        .sum()
}

fn collect_with_count<T: Eq + std::hash::Hash>(
    iter: impl Iterator<Item = (T, usize)>,
) -> HashMap<T, usize> {
    let hint = iter.size_hint();
    let mut new = HashMap::with_capacity(hint.1.unwrap_or(hint.0));

    iter.for_each(|(v, count)| {
        *new.entry(v).or_insert(0) += count;
    });

    new
}

common::read_main!();

#[test]
fn example() {
    let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    assert_eq!(solve(input), 525152);
}
