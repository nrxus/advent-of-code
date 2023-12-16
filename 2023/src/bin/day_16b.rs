use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let input = input.trim();
    let num_cols = input.lines().map(|l| l.len()).next().unwrap();
    let grid = input.as_bytes();
    let num_rows = (grid.len() + 1) / (num_cols + 1);

    (0..num_rows)
        .flat_map(|y| {
            [
                ((0, y), Direction::Right),
                ((num_cols - 1, y), Direction::Left),
            ]
        })
        .chain((0..num_cols).flat_map(|x| {
            [
                ((x, 0), Direction::Down),
                ((x, num_rows - 1), Direction::Up),
            ]
        }))
        .map(|start| solve_from_start(start, grid, num_cols, num_rows))
        .max()
        .unwrap()
}

fn solve_from_start(
    start: ((usize, usize), Direction),
    grid: &[u8],
    num_cols: usize,
    num_rows: usize,
) -> usize {
    let mut explored: HashSet<((usize, usize), Direction)> = HashSet::from_iter([start]);
    let mut beams = vec![start];

    let up = |(x, y): (usize, usize)| y.checked_sub(1).map(|y| ((x, y), Direction::Up));
    let down = |(x, y): (usize, usize)| Some(((x, y + 1), Direction::Down));
    let left = |(x, y): (usize, usize)| x.checked_sub(1).map(|x| ((x, y), Direction::Left));
    let right = |(x, y): (usize, usize)| Some(((x + 1, y), Direction::Right));

    while let Some((coord, direction)) = beams.pop() {
        let next = match grid[coord.1 * (num_cols + 1) + coord.0] {
            b'.' => [
                match direction {
                    Direction::Up => up(coord),
                    Direction::Down => down(coord),
                    Direction::Left => left(coord),
                    Direction::Right => right(coord),
                },
                None,
            ],
            b'|' => match direction {
                Direction::Up => [up(coord), None],
                Direction::Down => [down(coord), None],
                Direction::Left | Direction::Right => [up(coord), down(coord)],
            },
            b'\\' => match direction {
                Direction::Up => [left(coord), None],
                Direction::Down => [right(coord), None],
                Direction::Left => [up(coord), None],
                Direction::Right => [down(coord), None],
            },
            b'/' => match direction {
                Direction::Up => [right(coord), None],
                Direction::Down => [left(coord), None],
                Direction::Left => [down(coord), None],
                Direction::Right => [up(coord), None],
            },
            b'-' => match direction {
                Direction::Left => [left(coord), None],
                Direction::Right => [right(coord), None],
                Direction::Up | Direction::Down => [left(coord), right(coord)],
            },
            _ => unreachable!(),
        }
        .into_iter()
        .flatten()
        .filter(|((x, y), _)| *x < num_cols && *y < num_rows)
        .filter(|x| explored.insert(*x));

        beams.extend(next);
    }

    explored
        .into_iter()
        .map(|(c, _)| c)
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

common::read_main!();

#[test]
fn example() {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    assert_eq!(solve(input), 51);
}
