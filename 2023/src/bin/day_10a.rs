use std::collections::{HashMap, VecDeque};

fn solve(input: &str) -> i32 {
    let mut start = None;
    let map: HashMap<_, _> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                let pipe = match c {
                    '|' => Some(Pipe::NorthSouth),
                    '-' => Some(Pipe::EastWest),
                    'L' => Some(Pipe::NorthEast),
                    'J' => Some(Pipe::NorthWest),
                    '7' => Some(Pipe::SouthWest),
                    'F' => Some(Pipe::SouthEast),
                    'S' => Some(Pipe::Starting),
                    '.' => None,
                    _ => unreachable!(),
                }?;
                Some(((x, y), pipe))
            })
        })
        .inspect(|((x, y), pipe)| {
            if matches!(pipe, Pipe::Starting) {
                start = Some((*x, *y));
            }
        })
        .collect();

    let start = start.unwrap();

    // explore all directions at the start -- we don't know the pipe
    // shape but hopefully this works?
    let mut frontier = VecDeque::from_iter(
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
        .map(|d| (start, d, 0)),
    );

    while let Some((node, direction, steps)) = dbg!(frontier.pop_back()) {
        if steps != 0 && node == start {
            return steps / 2;
        }

        let Some(pipe) = dbg!(map.get(&node)) else { continue };

        let direction = match (pipe, direction) {
            (Pipe::NorthSouth, Direction::North) => Direction::North,
            (Pipe::NorthWest, Direction::East) => Direction::North,
            (Pipe::NorthEast, Direction::West) => Direction::North,
            (Pipe::NorthSouth, Direction::South) => Direction::South,
            (Pipe::SouthWest, Direction::East) => Direction::South,
            (Pipe::SouthEast, Direction::West) => Direction::South,
            (Pipe::SouthEast, Direction::North) => Direction::East,
            (Pipe::EastWest, Direction::East) => Direction::East,
            (Pipe::NorthEast, Direction::South) => Direction::East,
            (Pipe::EastWest, Direction::West) => Direction::West,
            (Pipe::NorthWest, Direction::South) => Direction::West,
            (Pipe::SouthWest, Direction::North) => Direction::West,
            (Pipe::Starting, d) => d,
            _ => continue,
        };

        let Some(next) = (match dbg!(direction) {
            Direction::North => node.1.checked_sub(1).map(|up| (node.0, up)),
            Direction::South => Some((node.0, node.1 + 1)),
            Direction::West => node.0.checked_sub(1).map(|left| (left, node.1)),
            Direction::East => Some((node.0 + 1, node.1)),
        }) else {
            continue;
        };

        frontier.push_front((dbg!(next), direction, steps + 1))
    }

    panic!("did not find loop")
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Starting,
}

common::read_main!();

#[test]
fn example_one() {
    let input = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
    assert_eq!(solve(input), 4);
}

#[test]
fn example_two() {
    let input = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    assert_eq!(solve(input), 8);
}
