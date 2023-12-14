fn solve(input: &str) -> usize {
    input.trim().split("\n\n").map(summarize).sum()
}

fn summarize(pattern: &str) -> usize {
    if let Some(score) = try_horizontal(pattern) {
        return score * 100;
    }

    let cols = pattern.lines().count();
    let rows = pattern.lines().next().unwrap().len();
    let mut transposed = vec![vec!['0'; cols]; rows];

    pattern
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .for_each(|(x, y, c)| transposed[x][y] = c);

    let transpoed_refs: Vec<_> = transposed.iter().map(|t| t.as_slice()).collect();
    check_reflection(&transpoed_refs).expect(pattern)
}

fn try_horizontal(pattern: &str) -> Option<usize> {
    let lines: Vec<&[u8]> = pattern.lines().map(|s| s.as_bytes()).collect();
    check_reflection(&lines)
}

fn check_reflection<'t, T: PartialEq>(lines: &[&[T]]) -> Option<usize> {
    // if the length is odd then skip the first element otherwise we'd
    // be reflecting in the middle of a line instead of in between two
    // liens
    let start = lines.len() % 2;
    // avoid reflecting on itself
    let end = lines.len() - 1;
    // skip the odd numbers
    for i in (start..end).step_by(2) {
        let mid = (lines.len() - i) / 2;
        let differences = lines
            .iter()
            .take(mid)
            .zip(lines.iter().rev().skip(i).take(mid))
            .flat_map(|(a, b)| a.into_iter().zip(b.into_iter()).filter(|(a, b)| a != b))
            .count();

        if differences == 1 {
            return Some(mid);
        }

        let differences = lines
            .iter()
            .skip(i)
            .take(mid)
            .zip(lines.iter().rev().take(mid))
            .flat_map(|(a, b)| a.into_iter().zip(b.into_iter()).filter(|(a, b)| a != b))
            .count();

        if differences == 1 {
            return Some(mid + i);
        }
    }

    None
}

common::read_main!();

#[test]
fn example() {
    let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!(solve(input), 400);
}

#[test]
fn example_two() {
    let input = r"#####..
..#....
...####
..#....
...#.##
...#...
#..#.##
";
    assert_eq!(solve(input), 1);
}

// 33605 too high

// 500 -> 1
