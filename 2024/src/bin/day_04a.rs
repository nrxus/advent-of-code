use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let wordsearch: HashMap<_, _> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    wordsearch
        .iter()
        .map(|(&k, &v)| (k, v))
        .filter_map(|(pos, c)| Some(pos).filter(|_| c == b'X'))
        .flat_map(|x_pos| {
            valid_neighbors(x_pos, &wordsearch)
                .filter_map(|(pos, c)| Some(pos).filter(|_| c == b'M'))
                .map(move |m_pos| (m_pos, (m_pos.0 - x_pos.0, m_pos.1 - x_pos.1)))
        })
        .filter_map(|(m_pos, delta)| {
            let a_pos = (m_pos.0 + delta.0, m_pos.1 + delta.1);
            wordsearch.get(&a_pos).filter(|c| **c == b'A')?;
            Some((a_pos, delta))
        })
        .filter_map(|(a_pos, delta)| {
            let s_pos = (a_pos.0 + delta.0, a_pos.1 + delta.1);
            wordsearch.get(&s_pos).filter(|c| **c == b'S')?;
            Some((s_pos, delta))
        })
        .count()
}

fn valid_neighbors(
    (x, y): (isize, isize),
    wordsearch: &HashMap<(isize, isize), u8>,
) -> impl Iterator<Item = ((isize, isize), u8)> + use<'_> {
    [
        // x - 1
        x.checked_sub(1).zip(y.checked_sub(1)),
        x.checked_sub(1).map(|x| (x, y)),
        x.checked_sub(1).map(|x| (x, y + 1)),
        // x
        y.checked_sub(1).map(|y| (x, y)),
        Some((x, y + 1)),
        // x + 1
        y.checked_sub(1).map(|y| (x + 1, y)),
        Some((x + 1, y)),
        Some((x + 1, y + 1)),
    ]
    .into_iter()
    .flatten()
    .filter_map(|pos| wordsearch.get(&pos).map(|v| (pos, *v)))
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
        ),
        18
    );
}
