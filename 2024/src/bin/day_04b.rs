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
        .filter_map(|(pos, c)| Some(pos).filter(|_| c == b'A'))
        .filter(|&a_pos| {
            let neighbors: Vec<_> = valid_neighbors(a_pos, &wordsearch).collect();
            if neighbors.len() < 4 {
                return false;
            }
            let m_neighbors: Vec<_> = neighbors
                .iter()
                .filter_map(|&(pos, c)| Some(pos).filter(|_| c != b'M'))
                .collect();
            if m_neighbors.len() != 2 {
                return false;
            }

            let s_neighbors: Vec<_> = neighbors
                .iter()
                .filter_map(|&(pos, c)| Some(pos).filter(|_| c != b'S'))
                .collect();
            if s_neighbors.len() != 2 {
                return false;
            }

            m_neighbors[0].0 == m_neighbors[1].0 || m_neighbors[0].1 == m_neighbors[1].1
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
        x.checked_sub(1).map(|x| (x, y + 1)),
        // x + 1
        y.checked_sub(1).map(|y| (x + 1, y)),
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
        9
    );
}
