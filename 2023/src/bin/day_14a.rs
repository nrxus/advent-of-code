fn solve(input: &str) -> usize {
    let num_cols = input.trim().lines().map(|l| l.len()).next().unwrap();

    let mut platform: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                'O' => State::Circle,
                '#' => State::Block,
                '.' => State::Empty,
                _ => unreachable!(),
            })
        })
        .collect();

    for i in num_cols..platform.len() {
        let State::Circle = platform[i] else { continue };
        let mut above = i - num_cols;
        while let State::Empty = platform[above] {
            let (first, second) = platform.split_at_mut(above + 1);
            std::mem::swap(&mut first[above], &mut second[num_cols - 1]);
            let Some(x) = above.checked_sub(num_cols) else {
                break;
            };
            above = x;
        }
    }

    let num_rows = platform.len() / num_cols;

    platform
        .chunks(num_cols)
        .enumerate()
        .map(|(y, row)| (num_rows - y) * row.iter().filter(|s| matches!(s, State::Circle)).count())
        .sum()
}

enum State {
    Circle,
    Block,
    Empty,
}

common::read_main!();

#[test]
fn example() {
    let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    assert_eq!(solve(input), 136);
}
