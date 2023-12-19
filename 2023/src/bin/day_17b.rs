use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

fn solve(input: &str) -> u32 {
    let input = input.trim();
    let num_cols = input.lines().map(|l| l.len()).next().unwrap();
    let map: Vec<_> = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let num_rows = map.len() / num_cols;

    // right or down is probably fine
    let start = Node {
        pos: (0, 0),
        direction: Direction::Right,
        in_a_row: 0,
    };
    let mut frontier = BinaryHeap::from_iter([(cmp::Reverse(0), start)]);
    let mut explored: HashSet<Node> = HashSet::from_iter([start]);
    let end = (num_cols - 1, num_rows - 1);

    while let Some((cmp::Reverse(old_cost), node)) = frontier.pop() {
        if node.pos == end {
            if node.in_a_row >= 4 {
                return old_cost;
            }
        }

        let mut next = [None, None, None];
        if node.in_a_row < 4 {
            next[0] = Some((node.direction, node.in_a_row + 1));
        } else {
            let turns = node.direction.turns();
            next[0] = Some((turns[0], 1));
            next[1] = Some((turns[1], 1));
            if node.in_a_row < 10 {
                next[2] = Some((node.direction, node.in_a_row + 1));
            }
        }

        let next = next
            .into_iter()
            .flatten()
            .filter_map(|(direction, in_a_row)| {
                let (x, y) = node.pos;
                match direction {
                    Direction::Up => y.checked_sub(1).map(|y| (x, y)),
                    Direction::Down => Some(y + 1).filter(|y| *y < num_rows).map(|y| (x, y)),
                    Direction::Left => x.checked_sub(1).map(|x| (x, y)),
                    Direction::Right => Some(x + 1).filter(|x| *x < num_cols).map(|x| (x, y)),
                }
                .map(|pos| Node {
                    pos,
                    direction,
                    in_a_row,
                })
            })
            .filter(|node| explored.insert(*node))
            .map(|node| {
                let (x, y) = node.pos;
                let next_cost = map[y * num_rows + x];
                (cmp::Reverse(old_cost + next_cost), node)
            });

        frontier.extend(next);
    }

    panic!("did not find path")
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pos: (usize, usize),
    direction: Direction,
    in_a_row: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turns(self) -> [Self; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

common::read_main!();

#[test]
fn example() {
    let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    assert_eq!(solve(input), 94);
}
