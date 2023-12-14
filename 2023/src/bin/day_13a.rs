fn solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| summarize(pattern))
        .sum()
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

    check_reflection(&transposed).expect(pattern)
}

fn try_horizontal(pattern: &str) -> Option<usize> {
    let lines: Vec<&str> = pattern.lines().collect();
    check_reflection(&lines)
}

fn check_reflection<T: PartialEq + std::fmt::Debug>(lines: &[T]) -> Option<usize> {
    let first = lines.first().unwrap();
    let last = lines.last().unwrap();

    for (i, (next, next_back)) in lines.iter().zip(lines.iter().rev()).enumerate() {
        if i == lines.len() - 1 {
            break;
        }

        if first == next_back {
            let mid = (lines.len() - i) / 2;
            let passes = lines
                .iter()
                .take(mid)
                .zip(lines.iter().rev().skip(i).take(mid))
                .all(|(a, b)| a == b);

            if passes {
                return Some(mid);
            }
        }

        if last == next {
            // verify that the inner ones also match
            let mid = (lines.len() - i) / 2;
            let passes = lines
                .iter()
                .skip(i)
                .take(mid)
                .zip(lines.iter().rev().take(mid))
                .all(|(a, b)| a == b);

            if passes {
                return Some(mid + i);
            }
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
    assert_eq!(solve(input), 405);
}

#[test]
fn example_horz() {
    let input = r"#...##..#
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
fn example_horz_rev() {
    let input = r"#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
#...##..#
";
    assert_eq!(solve(input), 300);
}

#[test]
fn example_harder() {
    let input = r"##.##.##......#
##....####..###
........#....#.
..#..#.........
..####..#....#.
##.##.##.#..#.#
#..##..########
.#....#.#....#.
..#####.######.
.........#..#..
.##..##..#..#..
.##..##..####..
..####..######.
.######.##..##.
.##..##.##..##.
#.#..#.#.####.#
#..##..#......#";
    assert_eq!(solve(input), 11);
}
