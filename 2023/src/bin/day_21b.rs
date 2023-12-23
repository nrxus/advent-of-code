use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let input = input.trim();
    let num_cols = input.lines().map(|l| l.len()).next().unwrap() as i64;
    let num_rows = input.lines().count() as i64;

    let mut start = None;
    let gardens: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c))
        })
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
    let mut explored: HashSet<((i64, i64), bool)> = HashSet::from_iter([(start, true)]);
    let mut frontier = VecDeque::from_iter([(start, 0)]);

    while let Some(((x, y), num_steps)) = frontier.pop_front() {
        println!("{num_steps}");
        if num_steps == NUM_STEPS {
            continue;
        }
        let neighbors = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .into_iter()
            .filter(|pos| {
                gardens.contains(&(pos.0.rem_euclid(num_cols), pos.1.rem_euclid(num_rows)))
            })
            .map(|pos| (pos, num_steps + 1))
            .filter(|&(pos, steps)| explored.insert((pos, steps % 2 == 0)));

        frontier.extend(neighbors);
    }

    explored
        .into_iter()
        .filter(|(_, is_even)| *is_even == (NUM_STEPS % 2 == 0))
        .count()
}

#[cfg(not(test))]
const NUM_STEPS: usize = 26501365;
#[cfg(test)]
const NUM_STEPS: usize = 100;

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
    assert_eq!(solve(input), 6536);
}
