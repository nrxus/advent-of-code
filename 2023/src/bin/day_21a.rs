use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let mut start = None;

    let gardens: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(move |(y, row)| row.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter_map(|(pos, c)| match c {
            '.' => Some(pos),
            'S' => {
                start = Some(pos);
                Some(pos)
            }
            '#' => None,
            _ => unreachable!(),
        })
        .collect();

    let start = start.unwrap();
    let mut explored: HashSet<((usize, usize), usize)> = HashSet::from_iter([(start, 0)]);
    let mut frontier = VecDeque::from_iter([(start, 0)]);

    while let Some(((x, y), num_steps)) = frontier.pop_back() {
        if num_steps == NUM_STEPS {
            continue;
        }
        let neighbors = [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some((x + 1, y)),
            Some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|pos| gardens.contains(pos))
        .map(|pos| (pos, num_steps + 1))
        .filter(|next| explored.insert(*next));

        frontier.extend(neighbors);
    }

    explored
        .into_iter()
        .filter(|(_, steps)| *steps == NUM_STEPS)
        .count()
}

#[cfg(not(test))]
const NUM_STEPS: usize = 64;
#[cfg(test)]
const NUM_STEPS: usize = 6;

common::read_main!();

#[test]
fn example() {
    let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
    assert_eq!(solve(input), 16);
}
