use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> isize {
    let input = input.trim();
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut robot_pos = None;
    let mut map: HashMap<(isize, isize), Tile> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .flat_map(|b| match b {
                    b'@' => [b'@', b'.'],
                    b'O' => [b'[', b']'],
                    b => [b, b],
                })
                .enumerate()
                .filter(|(_, b)| *b != b'.')
                .map(move |(x, b)| ((x as isize, y as isize), b))
        })
        .filter_map(|(pos, c)| {
            match c {
                b'@' => {
                    assert!(robot_pos.replace(pos).is_none());
                    None
                }
                b'#' => Some(Tile::Wall),
                b'[' => Some(Tile::LeftBox),
                b']' => Some(Tile::RightBox),
                _ => unreachable!(),
            }
            .map(|t| (pos, t))
        })
        .collect();

    let mut robot_pos = robot_pos.unwrap();
    instructions.lines().flat_map(|l| l.bytes()).for_each(|m| {
        let direction = match m {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => unreachable!(),
        };

        let wanted = (robot_pos.0 + direction.0, robot_pos.1 + direction.1);

        let mut boxes = if let Some(tile) = map.get(&wanted) {
            let b = match tile {
                Tile::LeftBox => Box {
                    y: wanted.1,
                    left: wanted.0,
                },
                Tile::RightBox => Box {
                    y: wanted.1,
                    left: wanted.0 - 1,
                },
                Tile::Wall => return,
            };
            HashSet::from_iter([b])
        } else {
            HashSet::new()
        };

        let mut to_move = vec![];
        while !boxes.is_empty() {
            to_move.extend(boxes.iter().cloned());

            let mut next = HashSet::new();

            for (tile, wanted) in boxes
                .into_iter()
                .flat_map(|b| match direction {
                    (0, dy) => vec![(b.left, b.y + dy), (b.left + 1, b.y + dy)],
                    (1, 0) => vec![(b.left + 2, b.y)],
                    (-1, 0) => vec![(b.left - 1, b.y)],
                    _ => unreachable!(),
                })
                .filter_map(|p| map.get(&p).map(|t| (t, p)))
            {
                let b = match tile {
                    Tile::LeftBox => Box {
                        y: wanted.1,
                        left: wanted.0,
                    },
                    Tile::RightBox => Box {
                        y: wanted.1,
                        left: wanted.0 - 1,
                    },
                    Tile::Wall => return,
                };

                next.insert(b);
            }

            boxes = next;
        }

        to_move.into_iter().rev().for_each(|b| {
            let (lp, lt) = map.remove_entry(&(b.left, b.y)).unwrap();
            let (rp, rt) = map.remove_entry(&(lp.0 + 1, lp.1)).unwrap();

            assert_eq!(lt, Tile::LeftBox);
            assert_eq!(rt, Tile::RightBox);

            let lp = (lp.0 + direction.0, lp.1 + direction.1);
            let rp = (rp.0 + direction.0, rp.1 + direction.1);
            assert!(map.insert(lp, lt).is_none());
            assert!(map.insert(rp, rt).is_none());
        });

        assert!(!map.contains_key(&wanted));
        robot_pos = wanted;
    });

    map.into_iter()
        .filter_map(|(pos, t)| {
            if matches!(t, Tile::LeftBox) {
                Some(pos)
            } else {
                None
            }
        })
        .map(|(x, y)| 100 * y + x)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Box {
    y: isize,
    left: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    LeftBox,
    RightBox,
    Wall,
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        ),
        9021
    );
}
