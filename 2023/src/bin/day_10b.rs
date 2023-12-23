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
                    x => unreachable!(),
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
        .map(|d| (start, d, vec![])),
    );

    let mut pipe_loop = None;
    // let mut foo = (None, None);
    while let Some((node, direction, mut steps)) = frontier.pop_back() {
        if !steps.is_empty() && node == start {
            pipe_loop = Some(steps);
            break;
        }

        let Some(pipe) = map.get(&node) else { continue };

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

        let Some(next) = (match direction {
            Direction::North => node.1.checked_sub(1).map(|up| (node.0, up)),
            Direction::South => Some((node.0, node.1 + 1)),
            Direction::West => node.0.checked_sub(1).map(|left| (left, node.1)),
            Direction::East => Some((node.0 + 1, node.1)),
        }) else {
            continue;
        };

        steps.push(node);
        frontier.push_front((next, direction, steps))
    }

    let pipe_loop = pipe_loop.unwrap();
    let pre_start = pipe_loop
        .get(1)
        .map(|&(x, y)| (x as i32, y as i32))
        .unwrap();
    let post_start = pipe_loop
        .last()
        .map(|&(x, y)| (x as i32, y as i32))
        .unwrap();
    let start = (start.0 as i32, start.1 as i32);
    let start_to_post = (post_start.0 - start.0, post_start.1 - start.1);
    let pre_to_start = (start.0 - pre_start.0, start.1 - pre_start.1);

    /*
    F 7
    L J

    - 7
      |

    F -
    |

      |
    _ J
     */

    let start_pipe = match (pre_to_start, start_to_post) {
        ((0, -1), (0, -1)) | ((0, 1), (0, 1)) => Pipe::NorthSouth,
        ((-1, 0), (-1, 0)) | ((1, 0), (1, 0)) => Pipe::EastWest,
        ((0, -1), (1, 0)) | ((0, 1), (-1, 0)) => Pipe::SouthEast,
        ((0, -1), (-1, 0)) | ((0, 1), (1, 0)) => Pipe::SouthWest,
        ((0, -1), (1, 0)) => Pipe::NorthEast,
        x => panic!("{x:?}"),
    };

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
#[ignore]
fn example_one() {
    let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
    assert_eq!(solve(input), 4);
}

#[test]
#[ignore]
fn example_two() {
    let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    assert_eq!(solve(input), 8);
}

#[test]
#[ignore]
fn example_three() {
    let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
    assert_eq!(solve(input), 10);
}
