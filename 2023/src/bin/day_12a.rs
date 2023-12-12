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
            permutate(readings, &counts)
        })
        .sum()
}

fn permutate(line: &str, desired: &[usize]) -> usize {
    let mut universes: Vec<(Vec<usize>, Option<usize>)> = vec![(vec![], None)];

    for c in line.chars() {
        match c {
            '.' => {
                universes.retain_mut(|(so_far, latest)| {
                    if let Some(latest) = latest.take() {
                        if desired.get(so_far.len()) != Some(&latest) {
                            return false;
                        }
                        so_far.push(latest)
                    }
                    true
                });
            }
            '#' => {
                universes.iter_mut().for_each(|(_, l)| match l {
                    Some(x) => *x += 1,
                    None => *l = Some(1),
                });
            }
            '?' => {
                let mut alt_universes = universes.clone();
                alt_universes.retain_mut(|(so_far, latest)| {
                    if let Some(latest) = latest.take() {
                        if desired.get(so_far.len()) != Some(&latest) {
                            return false;
                        }
                        so_far.push(latest)
                    }
                    true
                });

                universes.iter_mut().for_each(|(_, l)| match l {
                    Some(x) => *x += 1,
                    None => *l = Some(1),
                });

                universes.extend(alt_universes);
            }
            _ => unreachable!(),
        };
    }

    universes.retain_mut(|(so_far, latest)| {
        if let Some(latest) = latest.take() {
            if desired.get(so_far.len()) != Some(&latest) {
                return false;
            }
            so_far.push(latest)
        }
        true
    });

    universes.into_iter().filter(|(u, _)| *u == desired).count()
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
    assert_eq!(solve(input), 21);
}
