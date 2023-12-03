use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    let mut numbers = vec![];
    let mut maybe_gears = HashSet::new();

    input.trim().lines().enumerate().for_each(|(y, l)| {
        let mut remaining_line = l;
        let mut x = 0;

        while let Some(offset) = remaining_line.find(|c: char| c.is_digit(10)) {
            x += offset;

            let next = remaining_line.split_at(offset).1;
            let end = next.find(|c: char| !c.is_digit(10)).unwrap_or(next.len());
            let (number, next) = next.split_at(end);
            let len = number.len();

            let mut neighbors = HashSet::new();

            // same level left + right
            if let Some(left) = x.checked_sub(1) {
                neighbors.insert((left, y));
            }
            neighbors.insert((x + len, y));

            // level above
            if let Some(up) = y.checked_sub(1) {
                if let Some(left) = x.checked_sub(1) {
                    neighbors.insert((left, up));
                }
                neighbors.extend((x..=x + len).map(|x| (x, up)));
            }

            // level below
            if let Some(left) = x.checked_sub(1) {
                neighbors.insert((left, y + 1));
            }
            neighbors.extend((x..=x + len).map(|x| (x, y + 1)));

            let number: u32 = number.parse().unwrap();
            numbers.push((number, neighbors));
            x += len;
            remaining_line = next;
        }

        maybe_gears.extend(l.chars().enumerate().filter_map(|(x, c)| {
            if c == '*' {
                Some((x, y))
            } else {
                None
            }
        }))
    });

    maybe_gears
        .into_iter()
        .filter_map(|coord| {
            let matches: Vec<_> = numbers
                .iter()
                .filter_map(|(number, neighors)| {
                    if neighors.contains(&coord) {
                        Some(*number)
                    } else {
                        None
                    }
                })
                .take(3)
                .collect();

            if matches.len() != 2 {
                None
            } else {
                Some(matches[0] * matches[1])
            }
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(solve(input), 467835);
}
