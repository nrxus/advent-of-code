use std::collections::HashMap;

fn solve(input: &str) -> isize {
    let input = input.trim();
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut robot_pos = None;
    let mut map: HashMap<(isize, isize), Tile> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
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
                b'O' => Some(Tile::Box),
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

        let mut next = wanted;
        while let Some(tile) = map.get(&next).copied() {
            match tile {
                Tile::Box => {
                    next = (next.0 + direction.0, next.1 + direction.1);
                }
                Tile::Wall => return,
            }
        }

        if next != wanted {
            assert!(map.insert(next, Tile::Box).is_none());
        }

        map.remove(&wanted);
        robot_pos = wanted;
    });

    map.into_iter()
        .filter_map(|(pos, t)| {
            if matches!(t, Tile::Box) {
                Some(pos)
            } else {
                None
            }
        })
        .map(|(x, y)| 100 * y + x)
        .sum()
}

#[derive(Clone, Copy)]
enum Tile {
    Box,
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
        10092
    );
}

#[test]
fn example_small() {
    assert_eq!(
        solve(
            r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
        ),
        2028
    );
}
