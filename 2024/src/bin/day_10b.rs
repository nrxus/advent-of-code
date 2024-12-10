use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let map: HashMap<(isize, isize), u8> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, b)| ((x as isize, y as isize), b))
        })
        .collect();

    let next_step = |(x, y), altitude| {
        let map = &map;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(move |pos| {
                let value = map.get(&pos)?;
                if *value == altitude {
                    Some(pos)
                } else {
                    None
                }
            })
    };

    map.iter()
        .filter_map(|(pos, v)| if *v == b'0' { Some(*pos) } else { None })
        .flat_map(|pos| next_step(pos, b'1'))
        .flat_map(|pos| next_step(pos, b'2'))
        .flat_map(|pos| next_step(pos, b'3'))
        .flat_map(|pos| next_step(pos, b'4'))
        .flat_map(|pos| next_step(pos, b'5'))
        .flat_map(|pos| next_step(pos, b'6'))
        .flat_map(|pos| next_step(pos, b'7'))
        .flat_map(|pos| next_step(pos, b'8'))
        .flat_map(|pos| next_step(pos, b'9'))
        .count()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
        ),
        81
    );
}
